YOU EXPRESSLY ACKNOWLEDGE AND AGREE THAT USE OF THESE INSTRUCTIONS IS ARE YOUR SOLE RISK. AUTHORS OF THESE INSTRUCTIONS SHALL NOT BE LIABLE FOR DAMAGES OF ANY TYPE, WHETHER DIRECT OR INDIRECT.

# sachvo
Manage  iND neurons  without Internet Identity
## TLDR; 
This repository shows how to manage the staked neurons created in the NNS DAPP using Internet Identity (aka iND neurons) without using NNS DAPP (and hence Internet Identity). The management of staked neurons is limited to the features provided through the manage_neuron; which includes merging maturity. Additionally this management (particularly auto-merging) will NOT work for neurons created through Ledger Nano.

## Introduction
Post genesis (circa May 2021 ) the interest in Internet Computer has led to several people buying it's native token ICP. This native token can be staked in neurons to participate in active governance of the Internet Computer. The return on the stake is dependent on the length of time that the neuron is locked up. At the time of initial creation of this document (Dec 2021), the interest on staked ICP at the maximum lock up time of 8 years is 28% APY. There are alternate methods of staking. Some are suitable for highly technical audience while others have significantly better user experience in return for loss of full control. Prior to Dec 2021, there was no support for a true hardware wallet (like Ledger). Ledger support was added to ICP in Dec 2021. So people who staked neurons between May 2021 to Dec 2021 had two options: a. Highly Technical b. Ease of Use. Most people opted for the Ease of Use option. Some fairly significant investments have been made in staking through the Ease of Use option. 

As people begun to dig deeper into the mechanics of security behind the Ease of Use option after having taken that approach, some have begun to feel buyers remorse. This has been accentuated by some reported high loss by one or two people. The broad conclusion of the engineers and researchers working on the project has been to suggest that Internet Identity is not CURRENTLY (circa Dec 2021) suitable for storing signicant store of value. The nuances of this conclusion can be found in this extensive thread on forums at https://forum.dfinity.org/t/internet-identity-lack-of-security/9144. 

This repository illustrates a way to mitigate, post facto, the risks posed by using Internet Identity to manage the neurons created through the Ease of Use option. It should be noted that other alternatives to mitigate the risks do exist. The forum topic mentioned above is the best source of information about the alternatives. Some ofthe other alternatives are less burdensome than the alternative presented here. However this alternative follows the technical path of maximum protection in the situation presented. With the advent of Ledger Integration, most people in the future will not, hopefully, forced to choose between a. Highly Technical and b. Ease of Use option. They will choose Ledger Integration with NNS Dapp (aka lND neurons over iND neurons).

## Intended Audience 
The first focus is the audience which has staked neurons using the NNS DAPP using the Internet Identity and without the benefit of Ledger Nano. Secondly there are some aspects of this solution which might be thought-provoking to the curious.

## Ease of Use Option Explained
In the Ease of Use option, the native tokens of IC, ICP, are assumed to be procured through some third party exchange (i.e. coinbase). These native tokens are staked in the NNS for rewards gotten as a result of participation in governance. The act of staking is through the creation of the neuron which is locked for a duration of 6 months to eight years with a contribution of ICPs in the neuron. More ICPs, more rewards. Currently for a 8 year old neuron, the annual APY is roughly 28%. It is anticipated that this rate will go down with time as more ICPs are staked in the coming years. However for the purposes of this rep we will assume a constant rate of 28%. This is termed maturity in context of NNS. 

For illustration, if we assume 650 ICPs being staked in a 8 year non-dissolving neuron, currently we can assume a 0.5 ICP in maturity per day. A new neuron with applicable maturity can be spawned into a seperate neuron which then in turn can be dissolved; thereby producing newly minted ICP. Currently with this spawning can only happen when the maturity reaches 1 ICP or greater. In this illustration, the user has the ability to spawn a neuron every 2 days. The second alternative is to merge the maturity back into the orginal neuron. This can happen regardless of the reward in maturity (i.e. unlikely spawning which requires ICP, you can merge daily if you so desire). An astute observer will note that this is equivalent to daily compounding.

In order to merge daily, currently the user has to log into the NNS DAPP daily using the Internet Identity daily and manually do merge 100% on the neuron. Needless to say this is quite onerous for the user. If one does daily merges, the effective rate increases to 32% APY for the illustration. Some users do want to maximize their rate of returns. Some others fear using the Internet Identity on a daily basis (for the reasons in Introduction) for seeing the neuron every day to auto merge. Dfinity is working on a solution for auto-merging within governance; but this solution might take months. Another more short term solution is adding specialized hot-keys to manage the neuron through other principal.

This repository takes a different approach to solving the auto-merge maturity without ever need to login to NNS after some initial setup. 

## Outline of the solution

The core of the solution relies on the fact that it is possible for a neuron to follow certain other neuron for the topic of management. The initial attribution of thought is due to *skilesare* on *forum.dfinity.org* ; who identified this path. The topic of management lets the followee to propose merging maturity (amongst other things) for the follower. When such a neuron (aka follower) follows ONE other neuron (aka followee), the followee can float a proposal to merge maturity on the follower. Because there is only one followee, the proposal is immediately executed by the follower. 

Of course there are a few hindrances on implementation: 
 - how to create this type of a followee
 - how exactly to create such a proposal
 - how to productionize this 

## Roadmap of implementation

### Proof-of-concept 

#### Creation of Followee Neuron 
Create a *Followee* Neuron through command line. 

#### Creation of *Management* Followee
Assign the Followee Neuron to the Follower for the topic of Management
Provide the ability to *Unassign* the Followee

#### Creation of Merge Proposal through command line
Utilizing the dfx call ... manage_neuron command 

### Codification into POC
Develop code that formalizes this process.

### MVP
Cronify this code so that it is auto-merging daily.

### Auto Merging As A Service
 