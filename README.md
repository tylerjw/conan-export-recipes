# conan-export-recipes

Bulk export recipes in the folder structure of [conan-center-index](https://github.com/conan-io/conan-center-index)

1. Searches for and parses `config.yml` files
2. Executes `conan export {path} --version {version}` for each version of each recipe found

