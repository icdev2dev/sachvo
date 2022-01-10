# Summary
This document decribes the auto-merge functionality as implemented in the code. The details of why this functionaliity is built is shown in the README as well as how to set the management followee. 

# Pre-requisites 

## Technical

You need to have some technical skills to build this solution or have someone else build the solution for you. In particular you need to have some idea about how cron jobs work in Linux as well as have access to a Linux box from which this cron job will execute.

## Software

* rust 
* dfx installed at some path -- dfx-at-path


## NNS

* named identity through pem-file -- i.e pemid1
* Access to a properly funded command line created named neuron  -- i.e xxxxxxxxxxxx linked to pemid1



# Outline 

## 1 NNS Configuration

On the NNS Dapp, configure the neuron yyyyyyyy to follow *xxxxxxxxxx*  for management topic. See https://youtu.be/FfJLkjuLmaU for details on how to do.

## 2 Build the rust software 
Essentially cargo build

## 3 Configure auto-merge.txt

dfx-at-path:AUTOMERGE:100:pemid1:xxxxxxxxxx:yyyyyyyy

If there are more than one neuron (say yyyyyyyy and zzzzzzzz), have two lines

dfx-at-path:AUTOMERGE:100:pemid1:xxxxxxxxxx:yyyyyyyy
dfx-at-path:AUTOMERGE:100:pemid1:xxxxxxxxxx:zzzzzzzz