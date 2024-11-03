#!/bin/bash

set -euxo pipefail

echo $@

CFG_DIR=/cfg

# add CFG_DIR as first `looking dir` to allow to overrides commands.
mkdir -p $CFG_DIR
export PATH=$CFG_DIR:$PATH

cd $CFG_DIR
# see 0002-upgrade-node.zndsl to view the args.
curl -L -O $1/kvp
curl -L -O $1/kvp-prepare-worker
curl -L -O $1/kvp-execute-worker
chmod +x $CFG_DIR/kvp $CFG_DIR/kvp-prepare-worker $CFG_DIR/kvp-execute-worker
echo $(kvp --version)
