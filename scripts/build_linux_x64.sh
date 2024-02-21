########################
# Install necessary
# dependencies
########################
function install_dependencies() {
    apt-get update && apt-get install -y \
        lldb \
        build-essential \
        clang \
        libclang-dev \
        cmake \
        libprotobuf-dev \
        protobuf-compiler \
        libsuitesparse-dev \
        libeigen3-dev \
        curl \
        git \
        jq
}

########################
# Build and verify that
# unit-tests work.
########################
function build_and_test() {
    # AltDSS-Rust version follows
    # DSS CAPI version
    DSS_CAPI_VERSION=$(cargo metadata --format-version=1 --no-deps | jq '.packages[0].version' | tr -d '"')
    DSS_CAPI_PLATFORM="linux_x64"

    # Get dss_capi and necessary files
    wget -qO- "https://github.com/dss-extensions/dss_capi/releases/download/${DSS_CAPI_VERSION}/dss_capi_${DSS_CAPI_VERSION}_${DSS_CAPI_PLATFORM}.tar.gz" | tar zxv

    # If electricdss-tst doesn't exist,
    # clone it.
    if [[ ! -d "$PWD/electricdss-tst" ]];
    then    
        git clone --depth=1 https://github.com/dss-extensions/electricdss-tst
    fi  
    
    # Build and test.
    cargo build && \
    cargo test
}


install_dependencies
build_and_test