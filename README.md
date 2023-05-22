# Silph: A Framework for Scalable and Accurate Generation of Hybrid MPC Protocols

Edward Chen*, Jinhao Zhu*, Alex Ozdemir, Riad S. Wahby, Fraser Brown, Wenting Zheng. Silph: A Framework for Scalable and Accurate Generation of Hybrid MPC Protocols. IEEE S&P 2023.

This repository is currently under a snapshot from our submission. The latest implementation of 
Silph can be found in [CirC](https://github.com/circify/circ/tree/mpc_aws) on the *mpc_aws* branch.

# Features
To begin, set the desired features to `aby` which installs our fork of the [ABY](https://github.com/edwjchen/ABY) framework, 
`c` for the C frontend, and `lp` to enable the ILP solver. 

```bash
python3 driver.py -F aby c lp
```

# Installation 
To install the dependencies
```bash
python3 driver.py -i
```

# Build 
To build
```bash
python3 driver.py -b
```

# Benchmarks
To run our unit tests and benchmarks 
```bash
python3 driver.py -t
```

