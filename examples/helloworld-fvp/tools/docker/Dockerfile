FROM git.morello-project.org:5050/morello/morello-firmware-docker/morello-firmware-docker:latest

WORKDIR /root/workspace
USER root
RUN git config --global user.email "morello@builder" && \
    git config --global user.name "morello" && \
    repo init \
        -u https://git.morello-project.org/morello/manifest.git \
        -b morello/mainline \
        --no-repo-verify \
        -g bsp && \
        repo sync

RUN ./build-scripts/fetch-tools.sh -p fvp -f none build
RUN ./build-scripts/build-scp.sh -p fvp -f none all
RUN ./build-scripts/build-arm-tf.sh -p fvp -f none all

# TODO: (maybe) a Makefile rule that:
# - $(toolchain)-objcopy -O bin
# - rename it to `uefi.bin`
# - run the ./build-firmware-image.sh script to get fip.bin

ENTRYPOINT [""]
