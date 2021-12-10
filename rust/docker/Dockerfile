FROM rustembedded/cross:x86_64-unknown-linux-gnu-0.2.1

RUN apt-get update -y 

# Install openssl
RUN apt-get install -y libssl-dev 

# Install llvm & libclang
RUN apt-get install -y wget lsb-release software-properties-common apt-transport-https ca-certificates
RUN wget https://apt.llvm.org/llvm.sh && \
    chmod +x llvm.sh && \
    ./llvm.sh 12