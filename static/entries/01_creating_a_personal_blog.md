# Creating a basic on-prem personal blog
*Written on May 22nd, 2022*

You're in the middle of deciding what do you want to do. You go to GitHub, click 'explore' and see so many open-source projects in so many programming languages. You say to yourself 'agh, I really want to do stuff like that some day', so you decide that you're going to start a side project, right away. All these options, all these programming languages, all this topics, you want to learn **everything**! Where do we start!? 

Having so many options is definitely a double-edged sword. I often find myself either wanting to learn a language or wanting to do *X* thing. Be it to build a *DNS server* in Rust, a *game* with C++, my own *Docker* in Go or a *compiler* in Haskell. How great, fun stuff! With the power of internet and open-source information of nowadays, this isn't difficult to do. A simple Google or GitHub search and you're up and ready! 
But the thing is, **you have to decide** on what are you going to do. 

Say you *finally* decide on learning about *databases* with C. You've got the basics and found a nice blog post on creating a *SQLite clone* from scratch. You begin to read and immediately lose interest as it's like just copying code and seeing that you're not actually learning anything. Reading a bunch of words about *parsers* and *tokenizers* from a page on the internet doesn't help you understand a lot about the subject. 
That happened to me last week. I said to myself "I'm **going to finish** that project!" ... but just an hour later I was bored as hell. 

Maybe I'm more of a **research-and-break** guy. All of my side projects I've made began by just getting ideas on things I want to do and actually start coding something. No matter what it is, you type stuff, search documentation, and that slowly builds into a snowball. After *an hour* you realize you've been programming for *6 hours*. 
This, of course, works. Learning by doing is the #1 way to learn something. **Deciding** on something is the difficult part. 

So I've decided I'm going to build a simple **web server to host my blog**. My own stuff, where I'll do everything from scratch by **myself** and not following any tutorial. 

## Motivation 
The mix between university and work lets me learn *a lot* of stuff, and specially *fun* stuff. 
Docker, K8S, AWS, Nginx, Django, Cloud and security stuff and most importantly, the ability to **document** things and getting them done. 

Recently I have been watching a lot of YouTube, specially videos from **Pelado Nerd**, an Argentinian youtuber and SRE. Anything you want to learn about Cloud and Operations, he probably has a video on it. 
Also, there's one Twitch streamer that I found in the last weeks, **rwxrob**. It quickly became my favourite streamer as watching their streams is like learning by breathing. Seeing stuff about Docker, Kubernetes, Golang in such a deep way is awesome, while also creating a really nice community of cloud, linux, open-source-driven people. 
I've asked my teachers at university for some advice on what could I do with my love to Software Engineering and the desire to learn about Operating Systems. They told me I could see if **DevOps** is the right thing for me, so I'm giving it a try by creating this web server entirely from scratch with the things I have learned these past months, and I'm willing to even learn a lot more. 

## The plan 
In order to create it I'll need a couple of things: 
- A way to build my own infrastructure and setup a server 
- A way to manage my web resources (just a little, by creating custom URLs to sections of the web) 
- A way to convert Markdown into HTML (all of my blog posts will be written entirely in .md files) 
- A nice looking web page (god I hate front-end webdev, but whatever) 
- Set up a TLS certificate for the web  

I'll begin by creating some general visual structure of what my blog will look like. Not a lot of fancy stuff, just a basic way to display these words in a page with some styling. 
That could probably be done by a static generator site like *Hugo* or *Jekyll*, but how will I learn then? 
Then it's about creating a very simple *Django* or *Gin* backend stuff. The very basics to just show URLs, like **myweb.com/posts/**. 

Once that everything is done, that both front-end and back-end is working with a nice way to convert *.md* files to plain HTML, I'll setup an **Nginx web server** running *on-prem* with *TLS certification*. This step will probably be the most difficult one as I don't really have a lot of experience building infrastructure this way. But hell, it'll be fun.  

## The journey begins 
So, to sum up. I'll be doing a lot of stuff from scratch when I could simply use tools on the internet that do the job for me, like hosting on *github pages*, using a *static site generator* or letting *AWS* handle all of the infrastructure. But in that case I wouldn't be learning anything. That'll be like just reading a tutorial on how to create X thing, and we're back to the conflict! 
Of course, once I have learned how to do all of this stuff myself, in a next ocassion I'll *(probably)* just use the pre-written tools. 

It's a long journey. But **I will stick to the end**. Hopefully this post ends up online :) 

---
Related: 
* **Let's build a simple database in C**: https://cstack.github.io/db_tutorial/
* **Pelado Nerd**: https://www.youtube.com/c/PeladoNerd
* **rwxrob**: https://www.twitch.tv/rwxrob
---

> #blog #post #devops #project #backend #nginx #django
