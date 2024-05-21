# Getting Started with Innovating

## Setting up Private Fork

Innovators will want to create a private fork so that they can test that their algorithm can be successfully compiled into WASM by the CI.

1. Create private repository on GitHub
2. Create empty git repository on your local machine
    ```
    mkdir tig-monorepo
    cd tig-monorepo
    git init
    ```
3. Setup remotes with origin pointed to your private repository
    ```
    git remote add origin <your private repo>
    git remote add public https://github.com/tig-foundation/tig-monorepo.git
    ```
    
4. Pulling `blank_slate` from TIG public repository (branch with no algorithms)
    ```
    git fetch public
    git checkout -b blank_slate
    git pull public blank_slate
    ```
    
5. Push to your private repository
    ```
    git push origin blank_slate
    ```

## Checking out Existing Algorithms

Every algorithm has its own `<branch>` with name `<challenge_name>/<algorithm_name>`.

Only algorithms that are successfully compiled into WASM have their branch pushed to this public repository.

Each algorithm branch will have 6 key files:
1. Rust code with TIG commercial license header @ `tig-algorithms/src/<branch>/commercial.rs`
2. Rust code with TIG open data license header @ `tig-algorithms/src/<branch>/open_data.rs`
3. Rust code with TIG benchmarker outbound license header @ `tig-algorithms/src/<branch>/benchmarker_outbound.rs`
4. Rust code with TIG innovator outbound license header @ `tig-algorithms/src/<branch>/innovator_outbound.rs`
5. Rust code with TIG inbound license header @ `tig-algorithms/src/<branch>/inbound.rs`
6. Wasm blob @ `tig-algorithms/wasm/<branch>.wasm`

To pull an existing algorithm from TIG public repository, run the following command:
```
git fetch public
git pull public <branch>
```

## Developing Your Algorithm

1. Pick a challenge (`<challenge_name>`) to develop an algorithm for
2. Make a copy of `tig-algorithms/<challenge_name>/template.rs` or an existing algorithm (see notes)
3. Make sure your file has the following notice in its header if you intend to submit it to TIG:
```
Copyright [yyyy] [name of copyright owner]

Licensed under the TIG Inbound Game License v1.0 or (at your option) any later
version (the "License"); you may not use this file except in compliance with the
License. You may obtain a copy of the License at

https://github.com/tig-foundation/tig-monorepo/tree/main/docs/licenses

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the specific
language governing permissions and limitations under the License.
```
4. Rename the file with your own `<algorithm_name>`
5. Edit `tig-algorithms/<challenge_name>/mod.rs` to export your algorithm and test it:
    ```
    pub mod <algorithm_name>;

    #[cfg(test)]
    mod tests {
        use super::*;
        use tig_challenges::{<challenge_name>::*, *};

        #[test]
        fn test_<algorithm_name>() {
            let difficulty = Difficulty {
                // Uncomment the relevant fields.

                // -- satisfiability --
                // num_variables: 50,
                // clauses_to_variables_percent: 300,
                
                // -- vehicle_routing --
                // num_nodes: 40,
                // better_than_baseline: 250,
                
                // -- knapsack --
                // num_items: 50,
                // better_than_baseline: 10,
            };
            let seed = 0;
            let challenge = Challenge::generate_instance(seed, &difficulty).unwrap();    
            <algorithm_name>::solve_challenge(&challenge).unwrap();
        }
    }
    ```
6. Check that your algorithm compiles & runs:
    ```
    cargo test -p tig-algorithms
    ```

Notes:
* If you are copying and modifying an algorithm that has been submitted to TIG, make sure to use the `innovator_outbound` version
* Do not include tests in your algorithm file. TIG will reject your algorithm submission.
* Only your algorithm's rust code gets submitted. You should not be adding dependencies to `tig-algorithms` as they will not be available when TIG compiles your algorithm

## Locally Compiling Your Algorithm into WASM 

See the [README](../../tig-wasm/README.md) for `tig-wasm`

## Testing Performance of Algorithms

See the [README](../../tig-worker/README.md) for `tig-worker`

## Checking CI Successfully Compiles Your Algorithm

TIG pushes all algorithms to their own branch which triggers the CI (`.github/workflows/build_algorithm.yml`).

To trigger the CI on your private repo, your branch just needs to have a particular name:
```
git checkout -b <challenge_name>/<algorithm_name>
git push origin <challenge_name>/<algorithm_name>
```

## Making Your Submission

You will need to burn 0.001 ETH to make a submission. Visit https://play.tig.foundation/innovator and follow the instructions.