# Solver
Solver handles all of the math related function of parametrox.
---
## Core Goals
Solver has the following core goals
1. Define Parameter, a type for arbitrary expressions that can be evaluated at run time
2. Define Matrix, a type/struct with associated methods for performing most of the tasks one learns in an intro to linear algebra class
3. Define 2D/3D objects and assemblies through their assosciated types
4. Solve Ordinary and Partial differential Equations
---

# Current state
So far goal **1** is almost entirely complete, with some major room for optimization; goal **2** is nearing completion, with a few more methods needed to reach a good state; work has began on goal **3**, but there is a long road ahead; and practically 0 progress has been made towards goal **4**.

# File layout
 * **benches**: Contains various benchmarks written using Criterion
 * **examples**: Contains various examples of the solver crate
 * **src**
    * **geometry**
        * _constrain.rs_: Contains the actual CSG solver
        * _csg.rs_: contains the code to generate csg trees
        * _profile.rs_: contains the representation of 2d sketch
        * _sdf.rs_: contains the code to find the signed distance between primtives and sweeps
        * _sweep_.rs: defines the sweep operation
    * _function.rs_: defines the traits used by the parameter class, in the future possible contain code for diff eq solving
    * _geometry.rs_: bundles all of the files in the **geometry** folder together, creating the body and assembly structs
    * _lib.rs_: bundles the whole library together
    * _matrix.rs_: Defines a matrix structure and associated methods
    * _optimize.rs_: Contains the algorithms used by the CSG solver, namely newton-raphsom and analytic continuation(?)
    * _parameter.rs_: Defines the parameter type, which is is for numerical expressions that are defined and evaluated at runtime
    * _parse.rs_: Takes in raw strings like "3p0 + 1", and builds a parameter out of them
    * _vec.rs_: adds a couple of simple methods for vectors
 * **tests**: Contains various unit tests 
 * _architecture.md_: the document you are reading now 
 * _Cargo.lock_: Cargo.toml but for the compiler
 * _Cargo.toml_: defines dependencies and shi

# Next steps
 Right now, the next two big steps are to:
 1. Find an elegant way to represent infinite solutions to systems
 2. Implement Newton Raphsom, or something similar to it
## Representing infinite solutions to systems
 My issue is this, when I solve a system of linear equations, I want an elegant way to parameterize them and find solutions, while also being able to
 elegantly represent the single solutions too
 My main issue is that I dont know how to create a system that easily represents which variables are free
 I should probably reasearch how nalgebra does it
 My first reaction is to find a basis using the null space, colum space, but that doesnt solve my issue of not knowing which parameters are free
 I could do it so I have a basis where some of my vectors are 0, which would be redundant, but I can use to my advantage