# cellars

## what is going on here?
My own cute little project for environment managing and i guess lifecycle managing for the environments with an aim at simplification of install uninstall reinstall for dependancies and toolchains, as well as isolation of dependencies so i dont end up in dependency hell.

## how? ?
I don't really know. I don't even know how to structure this readme tbh. 
I actually JUST NOW like right now while writing this found out that conda can be used to manage environments with languages other than python. While programming python in high school i never really thought of the possibility because i just thought you know, anaconda - python. i never thought anaconda is based on package manager conda which does the environments ahh whatever. i'll just add conda as another backend option on this project (more on that [here.](#yr-really-that-riled-up-about-installing-tools-and-dependencies
))
After an intermediate level of research and about equal level of frustration on just existance, i decided to maybe write a cli tool that uses some package manager as a backend. I came upon nix -which is i guess notorious for having incredibly complex features?- and thought i could make a cli tool that kind of.. wraps? around nix and uses it to name different environments and use nix shells or nix envs to organize the workspace i am aiming for.

## now...
It's probably pretty obvious already if you've read till here that i am confused, and i don't know much about nix or conda or whatsoever. A more humane goal for me with this project is just to learn stuff. I haven't been programming for a very long time. I've wanted to get back into it and this seemed like a good project. The last time i was programming i was trying to learn rust and well, it felt fitting for a cli tool so i was like, oh great! If i can actually get it done it'd probably help propel me into actually programming because i'd have literally set up my own workspace with my own hands and could have it exactly adjusted to my needs.

this feels like a pretty personal project. and i feel i'll fck it up

## so... what's the plan?
First off, a cli tool. I called it cellars idk. thought i'd call each env a cellar. I should start simple but I kind of just want to add everything i think of at the beginning. Second off, I should make it a goal to write tests while writing code. I was never good at this. The idea for this was that.. well for context first, i just got a macbook and since i havent been programming its pretty much clean of any programming stuff. I dont want to install ANY tool or dependency on this machine. IDK WHY that's just how it is. i can't even get python to work like i want it to on homebrew because theres some weird shit about how something could mess up your system? or brew itself? boy i know nothing. 
But, i know i have to install stuff of course. so this tool is going to be something that keeps records of whats installed keep them in their own little bubbles (hopefully) and give me a nice cute little way of reaching those by typing a name i put to them. then when im done with that environment i can kill it off. but the records are kept so if i realize i want to get back to it or i for example pick a project i had stopped developing a record of what i need to jump back in is there and i can just woop rebuild the environment.
and if im really done with it just trash that record to and were good to go.
nix seems to be pretty cool. in fact i may not even really need this and this might be achievable with just nix and profiles. but 1. i dont know, 2. i still want to do this little lets see what i learn project.

## yr really that riled up about installing tools and dependencies?
Honestly, i don't know. But, i mean. I kind of eer on organization so.. yeah. And! i thought i could add a little thing to vim that uses this project so i could pop into a env through vim. Basically learn more stuff lol. And while we got that much done i thought why just wrap around nix? With that new found info about conda working with other programming language tools and with different types of virtual environments like docker images, etc. i thought you know? if i can actually get this done, why not add options and integrate those in here as well. and maybe i could use even homebrew ore whatever the default package manager is on any given machine and somehow have that package manager act in a way that cellars can use it to keep environments.

# to be continued
## i have to do a lot of research and i have a lot to learn. i'll update this doc.