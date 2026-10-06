#!/bin/bash
cat src/UserInterface/Packet.h | grep -B 2 -A 5 "command_party_set_state"
