# Contributing to siz

We welcome contributions to the `siz` project under the Apache License v2.0. Whether you're looking to fix bugs, add new features, or improve documentation, your help is greatly appreciated.

Here's how you can contribute:

1. **Fork the Repository**: Fork the `siz` repository to your GitHub account.

2. **Clone the Repository**: Clone the forked repository to your local machine.

    ```text
    git clone https://github.com/<your-username>/siz.git
    ```

3. **Create a New Branch**: Create a new branch for your changes.

    ```text
    git checkout -b add-new-feature
    ```

4. **Make Your Changes**: Make changes to the code or documentation.

5. **Commit Your Changes**: Commit your changes to your branch. Include a commit message that briefly describes your changes.

    ```text
    git commit -m "Add new feature"
    ```

6. **Push Your Changes**: Push your changes to your forked repository on GitHub.

    ```text
    git push origin add-new-feature
    ```

7. **Submit a Pull Request**: Follow the [GitHub Pull Request instructions](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request) to create a new pull request with your proposed changes.

**Before submitting a pull request**, please ensure your code compiles and all tests, including the clippy lints, pass. If you're adding a new feature, please add tests that cover the new feature.

You can run the following commands in the root of your downstream source repository to execute these tests before you submit your pull request:

```text
cargo test
cargo clippy
```

Please feel free to reach out if you have any questions or need help contributing.
