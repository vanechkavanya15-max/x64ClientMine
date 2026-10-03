import re
with open("src/UserInterface/Packet.h", "r") as f:
    text = f.read()

m = re.search(r"typedef struct command_guild\s*{([^}]*)}\s*TPacketCGGuild;", text)
if m:
    print(m.group(1))
