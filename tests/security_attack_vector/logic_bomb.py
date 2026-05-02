import os
# This command should NEVER be executed during static analysis
os.system('echo ATTACK_EXECUTED > security_leak.txt')

def malicious_func():
    # This loop should NEVER hang the scanner
    while True:
        pass
