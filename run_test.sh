mkdir -p mock_build && cd mock_build
# To bypass linking errors with missing DirectX stuff in this container for a single unit test
# we compile the necessary objects manually. Wait, Doctest main file must be built too.
