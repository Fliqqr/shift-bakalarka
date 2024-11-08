\title{\Huge \textbf{Software-Implemented Hardware Fault Tolerance}}
\author{Filip Ďuriš}

\maketitle

\newpage

# 1. Hardware Errors

# 2. Fault tolerant software

## 2.1 Single version vs multi-version

## 2.2 Single version

Single version is a technique which focuses on adding safety checks and redundancies into a singular version of the software, that make it the most resilient against error and external factors.
This includes detection of faults and ability to recover from them.

## 2.2.1 Modularity

## 2.2.2 Error detection

## 2.2.3 Exception handling

## 2.2.4 Checkpoint and restart

## 2.2.5 Process pairs

## 2.3 Multi-version

Multi-version relies on multiple different version of the same software, where the failure of a single variant does not have an impact on the overall system.
Multiple versions of the same software are exectues either in sequence or in parallel, each utilizing different error detection and recovery, to have the higest probability of at least one completing the task successfully.

### 2.3.1 Recovery blocks

Recovery blocks uses the basic principles of single version techniques to detect errors, wherein upon detection, recovery blocks will select a different version of the software for execution to recover from an error. Hence it is an extension of multi-version programming.

Benefit of recovery blocks is the fact, that in most cases the first execution will be succesful, meaning the subsequent version of the sofware can be designed with degrading performance to ensure maximal redundancy and safety.

Thanks to errors being relatively rare compared to normal execution of the code, this approach can achieve both peformance and reliability in most situations.

### 2.3.2 N-version programming

N-version programming is an extension of multi-version technique, where the same task is peformed N-time in parallel and the resulting outcome is decided through a concensus algorithm between the results of all N executions. This is done usually through a voter algorithm.

### 2.3.3 N Self-checking programming

### 2.3.4 t/(n-1)-Variant programming
