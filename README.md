# Chaz Tunnel - an SSH tunneling service for Chaz the NAS
This repository contains a collection of test files and some support tools for setting up an SSH tunneling service for my home NAS. My aims for this project are relatively simple: 

1. *Be able to poll the state of SSH-able computers on my home network.* I think long term this could be expanded to encapsulate the ability to schedule jobs on home machines through Chaz for monitoring or otherwise, but I don't have a lot of use cases for this yet. 

2. *Be able to connect via SSH to my machines through Chaz*. This one is simpler, after I know that any given machine is discoverable on my network, I want the abiltiy to connect to it through Chaz, with ideally as little latency as possible. I'm unsure whether or not its worth tunnelling through Chaz, but it feels like an easy neck down point where Chaz can enforce certain security requirements or otherwise log activity in the event of security vulnerabilities.

## Development Start 
To get started, I'm going to try and create a simulated environment of what my ideal behavior would be, using Docker containers. My goal for this first test is to see if I can get a "mainServer" acting node to poll a handful of dummy containers I spin up and report back random information, before tunnelling an SSH connection to them. 
