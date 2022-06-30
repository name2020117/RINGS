# HACK: This whole script is hacky, but it is just a trial to prove the
# concept works before refactoring my publisher and subscriber code.
# NOTE: nr-binder must be executable

import subprocess
from argparse import ArgumentParser

parser = ArgumentParser()
parser.add_argument("-n", "--n_ue", default=10, type=int, required=False)
parser.add_argument("-m", "--m_messages", default=10, type=int, required=False)
parser.add_argument("-s", "--sleep_time", default=1, type=int, required=False)
args = parser.parse_args()


for i in range(1, args.n_ue + 1):
    # simtun - 1 since it starts at 0
    print(f"echo ./build/nr-binder 10.60.0.{i:02} uesimtun{i-1} &")
    subprocess.run(
        f"./build/nr-binder 10.60.{i:02} ./pub -b 129.114.26.170"
        + f"-p 9092 -t test -n {args.m_messages} -s 1 &",
        text=True,
        check=False,
    )
