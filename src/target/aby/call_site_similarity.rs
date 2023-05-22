//! Call Site Similarity

use crate::ir::term::*;
use crate::target::aby::assignment::def_uses::*;

use fxhash::{FxHashMap, FxHashSet};

use std::collections::HashMap;
use std::collections::HashSet;

/// What do we need for call site?
///
/// Call sites:
/// HashMap<(String, Vec<usize>, Vec<usize>), Vec<Term>>  
/// - Each entry of {call_sites} will become a copy of function
///
/// Computations:
/// HashMap<String, HashMap<usize, Computation>>
/// - String: fname
/// - usize: version id
///
/// DefUseGraph:
/// HashMap<String, DefUseGraph>
///
/// Surrounding info:
/// Two type:
/// 1. For inner calls:
///     - Per call
/// 2. For outer calls:
///     - Per call site
///
/// args: HashMap<String, Vec<Term>>
/// rets: HashMap<String, Vec<Term>>

#[derive(Clone)]
/// A structure that stores the context and all the call terms in one call site
struct CallSite {
    // Context's fname
    pub arg_names: Vec<String>,
    pub args: Vec<Vec<Term>>,
    pub rets: Vec<Vec<Term>>,
    pub calls: Vec<Term>,
    pub caller_dug: DefUsesGraph,
}

impl CallSite {
    pub fn new(
        args: &Vec<Vec<Term>>,
        arg_names: &Vec<String>,
        rets: &Vec<Vec<Term>>,
        t: &Term,
        caller_dug: &DefUsesGraph,
    ) -> Self {
        Self {
            arg_names: arg_names.clone(),
            args: args.clone(),
            rets: rets.clone(),
            calls: vec![t.clone()],
            caller_dug: caller_dug.clone(),
        }
    }
}

/// Call site analysis
pub struct CallSiteSimilarity {
    fs: Functions,
    dugs: HashMap<String, DefUsesGraph>,
    visited: HashSet<String>,
    call_sites: HashMap<(String, Vec<usize>, Vec<usize>), CallSite>,
    callee_caller: HashSet<(String, String)>,
    func_to_cs: HashMap<String, HashMap<usize, CallSite>>,
    dup_per_func: HashMap<String, usize>,
    call_cnt: HashMap<String, usize>,
    ml: usize,
}

impl CallSiteSimilarity {
    /// Initialize
    pub fn new(fs: &Functions, ml:&usize) -> Self {
        let css = Self {
            fs: fs.clone(),
            dugs: HashMap::new(),
            visited: HashSet::new(),
            call_sites: HashMap::new(),
            callee_caller: HashSet::new(),
            func_to_cs: HashMap::new(),
            dup_per_func: HashMap::new(),
            call_cnt: HashMap::new(),
            ml: ml.clone(),
        };
        css
    }

    fn traverse(&mut self, fname: &String) {
        *self.call_cnt.entry(fname.clone()).or_insert(0) += 1;
        let c = self.fs.get_comp(fname).unwrap().clone();
        for t in c.terms_postorder() {
            if let Op::Call(callee, ..) = &t.op {
                self.traverse(callee);
            }
        }
        if !self.visited.contains(fname) {
            println!("Building dug for {}", fname);
            let mut dug = DefUsesGraph::for_call_site(&c, &self.dugs, fname);
            dug.gen_in_out(&c);
            let cs: Vec<(Term, Vec<Vec<Term>>, Vec<Vec<Term>>)> = dug.get_call_site();
            for (t, args_t, rets_t) in cs.iter() {
                if let Op::Call(callee,_, _, _) = &t.op {
                    // convert term to op id
                    let key: (String, Vec<usize>, Vec<usize>) =
                    (callee.clone(), to_key(args_t), to_key(rets_t));
                    if self.call_sites.contains_key(&key) {
                        self.call_sites.get_mut(&key).unwrap().calls.push(t.clone());
                    } else {
                        // Use the first context
                    if let Op::Call(_, arg_names, _, _) = &t.op {
                        let cs = CallSite::new( args_t, arg_names, rets_t, t, &dug);
                        self.call_sites.insert(key, cs);
                    }
                }
                // recording callee-caller
                self.callee_caller.insert((callee.clone(), fname.clone()));
            }
            }
            self.dugs.insert(fname.clone(), dug);
            self.dup_per_func.insert(fname.clone(), 0);
            self.func_to_cs.insert(fname.clone(), HashMap::new());
            self.visited.insert(fname.clone());
        }
    }

    /// Given a Computations
    /// Group function calls with identical/similar call site
    /// Rewrite the IR to differentiate functions with different call site
    pub fn call_site_similarity_smart(&mut self) -> (Functions, HashMap<String, DefUsesGraph>) {
        let main = "main".to_string();
        self.traverse(&main);
        // todo!("Testing");

        // Functions that have more than one call site
        let mut duplicated_f: HashSet<String> = HashSet::new();
        // Functions that need to be rewrote for calling to duplicated f
        // If a callee is duplicated, the caller need to be rewrote
        let mut rewriting_f: HashSet<String> = HashSet::new();
        let mut call_map: TermMap<usize> = TermMap::new();

        // Generating duplicate set
        for (key, cs) in self.call_sites.iter() {
            let call_id: usize = self.dup_per_func.get(&key.0).unwrap().clone();

            if call_id > 0 {
                // indicate this function need to be rewrote
                duplicated_f.insert(key.0.clone());
            }

            for t in cs.calls.iter() {
                call_map.insert(t.clone(), call_id);
            }
            self.dup_per_func.insert(key.0.clone(), call_id + 1);
            let id_to_cs = self.func_to_cs.get_mut(&key.0).unwrap();
            id_to_cs.insert(call_id, cs.clone());
        }


        // Generating rewriting set
        for (callee, caller) in self.callee_caller.iter() {
            if duplicated_f.contains(callee) {
                rewriting_f.insert(caller.clone());
            }
        }

        remap(
            &self.fs,
            &rewriting_f,
            &duplicated_f,
            &call_map,
            &self.call_cnt,
            &self.func_to_cs,
            self.ml,
        )
    }
}

/// Rewriting the call term to new call
fn rewrite_call(c: &mut Computation, call_map: &TermMap<usize>, duplicate_set: &HashSet<String>) {
    let mut cache = TermMap::<Term>::new();
    let mut children_added = TermSet::new();
    let mut stack = Vec::new();
    stack.extend(c.outputs.iter().cloned());
    while let Some(top) = stack.pop() {
        if !cache.contains_key(&top) {
            // was it missing?
            if children_added.insert(top.clone()) {
                stack.push(top.clone());
                stack.extend(top.cs.iter().filter(|c| !cache.contains_key(c)).cloned());
            } else {
                let get_children = || -> Vec<Term> {
                    top.cs
                        .iter()
                        .map(|c| cache.get(c).unwrap())
                        .cloned()
                        .collect()
                };
                let new_t_op: Op = match &top.op {
                    Op::Call(name, arg_names, arg_sorts, ret_sorts) => {
                        let mut new_t = top.op.clone();
                        if duplicate_set.contains(name) {
                            if let Some(cid) = call_map.get(&top) {
                                let new_n = format_dup_call(name, cid);
                                let mut new_arg_names: Vec<String> = Vec::new();
                                for an in arg_names.iter() {
                                    new_arg_names.push(an.replace(name, &new_n));
                                }
                                new_t = Op::Call(
                                    new_n,
                                    new_arg_names,
                                    arg_sorts.clone(),
                                    ret_sorts.clone(),
                                );
                            }
                        }
                        new_t
                    }
                    _ => top.op.clone(),
                };
                let new_t = term(new_t_op, get_children());
                cache.insert(top.clone(), new_t);
            }
        }
    }
    c.outputs = c
        .outputs
        .iter()
        .map(|o| cache.get(o).unwrap().clone())
        .collect();
}

/// Rewriting the var term to new name
fn rewrite_var(c: &mut Computation, fname: &String, cid: &usize) {
    let mut cache = TermMap::<Term>::new();
    let mut children_added = TermSet::new();
    let mut stack = Vec::new();
    stack.extend(c.outputs.iter().cloned());
    while let Some(top) = stack.pop() {
        if !cache.contains_key(&top) {
            // was it missing?
            if children_added.insert(top.clone()) {
                stack.push(top.clone());
                stack.extend(top.cs.iter().filter(|c| !cache.contains_key(c)).cloned());
            } else {
                let get_children = || -> Vec<Term> {
                    top.cs
                        .iter()
                        .map(|c| cache.get(c).unwrap())
                        .cloned()
                        .collect()
                };
                let new_t_op: Op = match &top.op {
                    Op::Var(name, sort) => {
                        let new_call_n = format_dup_call(fname, cid);
                        let new_var_n = name.replace(fname, &new_call_n);
                        Op::Var(new_var_n.clone(), sort.clone())
                    }
                    _ => top.op.clone(),
                };
                let new_t = term(new_t_op, get_children());
                cache.insert(top.clone(), new_t);
            }
        }
    }
    c.outputs = c
        .outputs
        .iter()
        .map(|o| cache.get(o).unwrap().clone())
        .collect();
}

fn traverse(fs: &Functions, fname: &String, dugs: &mut HashMap<String, DefUsesGraph>) {
    if !dugs.contains_key(fname) {
        let c = fs.get_comp(fname).unwrap().clone();
        for t in c.terms_postorder() {
            if let Op::Call(callee, ..) = &t.op {
                traverse(fs, callee, dugs);
            }
        }
        let mut dug = DefUsesGraph::for_call_site(&c, dugs, fname);
        dug.gen_in_out(&c);
        dugs.insert(fname.clone(), dug);
    }
}

fn remap(
    fs: &Functions,
    rewriting_set: &HashSet<String>,
    duplicate_set: &HashSet<String>,
    call_map: &TermMap<usize>,
    call_cnt: &HashMap<String, usize>,
    func_to_cs: &HashMap<String, HashMap<usize, CallSite>>,
    ml: usize,
) -> (Functions, HashMap<String, DefUsesGraph>) {
    let mut n_fs = Functions::new();
    let mut n_dugs: HashMap<String, DefUsesGraph> = HashMap::new();
    let mut context_map: HashMap<String, CallSite> = HashMap::new();
    let mut css_call_cnt: HashMap<String, usize> = HashMap::new();
    for (fname, comp) in fs.computations.iter() {
        let mut ncomp: Computation = comp.clone();
        let id_to_cs = func_to_cs.get(fname).unwrap();

        if rewriting_set.contains(fname) {
            rewrite_call(&mut ncomp, call_map, duplicate_set);
        }

        if duplicate_set.contains(fname) {
            for (cid, cs) in id_to_cs.iter() {
                let new_n: String = format_dup_call(fname, cid);
                let mut dup_comp: Computation = Computation {
                    outputs: ncomp.outputs().clone(),
                    metadata: rewrite_metadata(&ncomp.metadata, fname, &new_n),
                    precomputes: ncomp.precomputes.clone(),
                };
                rewrite_var(&mut dup_comp, fname, cid);
                n_fs.insert(new_n.clone(), dup_comp);
                context_map.insert(new_n.clone(), cs.clone());
                css_call_cnt.insert(new_n, call_cnt.get(fname).unwrap().clone());
            }
        } else {
            if let Some(cs) = id_to_cs.get(&0){
                context_map.insert(fname.clone(), cs.clone());
                css_call_cnt.insert(fname.clone(), call_cnt.get(fname).unwrap().clone());
            }
            n_fs.insert(fname.clone(), ncomp);
        }
    }
    let main = "main".to_string();
    traverse(&n_fs, &main, &mut n_dugs);

    for (fname, cs) in context_map.iter() {
        let dug = n_dugs.get_mut(fname).unwrap();
        let comp = n_fs.get_comp(fname).unwrap();
        dug.set_num_calls(css_call_cnt.get(fname).unwrap());
        dug.insert_context(&cs.arg_names, &cs.args, &cs.rets, &cs.caller_dug, comp, ml);
    }

    (n_fs, n_dugs)
}

fn format_dup_call(fname: &String, cid: &usize) -> String {
    format!("{}_circ_v_{}", fname, cid).clone()
}

fn rewrite_metadata(
    md: &ComputationMetadata,
    fname: &String,
    n_fname: &String,
) -> ComputationMetadata {
    let mut input_vis: FxHashMap<String, (Term, Option<PartyId>)> = FxHashMap::default();
    let mut computation_inputs: FxHashSet<String> = FxHashSet::default();
    let mut computation_arg_names: Vec<String> = Vec::new();

    for (s, tu) in md.input_vis.iter() {
        let s = s.clone();
        let new_s = s.replace(fname, n_fname);
        input_vis.insert(new_s, tu.clone());
    }

    for s in md.computation_inputs.iter() {
        let s = s.clone();
        let new_s = s.replace(fname, n_fname);
        computation_inputs.insert(new_s);
    }

    for s in md.computation_arg_names.iter() {
        let s = s.clone();
        let new_s = s.replace(fname, n_fname);
        computation_arg_names.push(new_s);
    }

    ComputationMetadata {
        party_ids: md.party_ids.clone(),
        next_party_id: md.next_party_id.clone(),
        input_vis,
        computation_inputs,
        computation_arg_names,
    }
}

fn to_key(vterms: &Vec<Vec<Term>>) -> Vec<usize> {
    let mut key: Vec<usize> = Vec::new();
    for terms in vterms{
        let mut v: Vec<usize> = Vec::new();
        for t in terms{
            v.push(get_op_id(&t.op));
        }
        v.sort();
        key.extend(v);
    }
    key
}

fn get_op_id(op: &Op) -> usize {
    match op {
        Op::Var(..) => 1,
        Op::Const(_) => 2,
        Op::Eq => 3,
        Op::Ite => 4,
        Op::Not => 5,
        Op::BoolNaryOp(_) => 6,
        Op::BvBinPred(_) => 7,
        Op::BvNaryOp(_) => 8,
        Op::BvBinOp(_) => 9,
        Op::Select => 10,
        Op::Store => 11,
        Op::Call(..) => 12,
        _ => todo!("What op?"),
    }
}
