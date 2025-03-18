# Software Name
Rust-based Leetcode solutions and code review repository.

## Programming Exercise Process

1. Create a personal repository

- Each participant should create their own repository.

- The `README.md` file can be customized freely.

- A `CHANGELOG.md` file is not required.

2. Issue and Pull Request Workflow

- Create an issue for each problem.

- Include the problem link in the issue description.

- Submit a Pull Request (PR) linked to the issue.

- Reviewers approve the PR before merging.

- The author of the PR is responsible for merging after approval.

3. Handling Code Revisions

- If a review results in changes that should also be applied to previously
merged code, create a separate issue and proceed accordingly.

4. Library Restrictions

- Do not use external libraries; only the Rust standard library is allowed.

5. Flexible Problem Scope

If a problem's requirements are ambiguous, decide on an approach individually.

The goal is **familiarity with Rust**, not just solving the problem.

## Coding Guidelines

- Maximize the use of Option and Result

  - Utilize Rust's built-in error handling mechanisms instead of relying on
    panics.

- Prioritize exception handling

  - Properly handle errors to ensure robust and safe code execution.

- Leverage standard Rust traits (std::trait)

  - Make use of Rust's standard traits to write idiomatic and reusable code.

- Use iterators instead of for loops

  - Prefer .iter(), .map(), .filter(), and other iterator methods to improve
    code clarity and efficiency.

- Avoid panic!, unwrap, and expect

  - Do not use unsafe error handling practices; refer to the documentation for
    alternatives.

- Minimize cloning (clone()) and mutable state (mut)

  - Reduce unnecessary heap allocations and mutable references to improve
    performance and maintainability.

- Optimize memory allocation

  - Use references (&, &mut) whenever possible instead of cloning or heap
    allocations to keep memory usage minimal.

## Why Naming & Branching Rules Are Not Applied

This repository is a personal coding exercise rather than an enterprise project,
so strict naming and branching/versioning rules are unnecessary.

- No multiple project branches: We don’t maintain custom versions for different
  users.

- No strict versioning: Semantic versioning (x.y.z+branch.version) is not
  required since this is not a deployable product.

- Simplified workflow: We follow a feature branch → PR → merge to main process.

- Branch name parsing: We use a simple convention like
  {your-name}/tis/{my-branch-name} for individual work branches.

However, commit message guidelines are enforced for clarity and consistency.

## License

Copyright 2024 ClumL Inc.

Licensed under [Apache License, Version 2.0][apache-license] (the "License");
you may not use this crate except in compliance with the License.

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See [LICENSE](LICENSE) for
the specific language governing permissions and limitations under the License.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the [Apache-2.0
license][apache-license], shall be licensed as above, without any additional
terms or conditions.

[apache-license]: http://www.apache.org/licenses/LICENSE-2.0