<!--
title: Better Axumbody...
date: 20220526
-->

Recently I started to mess around with the Axum library in Rust by the tokio people. After messing around with it with a few toy projects I must say I was pleasently surprised by how easy it was to get things up and running. So much so, that I thought I may as well migrate my blog that I never use to it. 

### downtime / update

The migration was relatively straightforward. Previously I was using actix, and that was fine. Don't get me wrong, but axum just felt a hellavalot better. While the library is still in its' infancy, the thought that has gone into how a user uses the library felt really nice. Nothing that I wouldn't expect from the developers of tokio, hyper, tower, and all those great libraries.

Basically, all I had to do was change the http service to axum, and rewrite the handlers. I'm pretty sure I actually reduced the code significantly. Which is always a good sign in my opinion.

### Something for the future?

One of the projects I've been working on in my own time is a remote access trojan built in Rust. It's more of a pet project just to up my skills in Rust, but it has been quite fun. At the moment I have a simple HTTP listener working and an agent which can run commands. I have a few features which I'd like to add, at some point, which I'll probably note down below. At least that way it'll be documented somewhere other than my forgetful brain. So here are some of the things planned:

* Micrservice architecture - Basically I want the C2 server to be separate from the listeners, and I was potentially thinking of having a separate service which consumes logs. I can't take credit for the idea though, I was only recently introduced to the mythic c2 framework, and it gave me the idea to separate the architecture.
* gRPC communication between modules and the C2 service. This will utilise RusTLS. Using the [tonic](https://github.com/hyperium/tonic) library.
* Desktop client - I'm hoping that I can implement a cross-platform desktop client for the C2, which utilises the [tauri](https://tauri.studio) library. Essentially, it's lightweight Electron, written in Rust. It allows users to use your very common JavaScript frameworks like React, Vue, etc.
* REST API - For interactions between desktop client and the C2 service. Using, you guessed it [axum](https://github.com/tokio-rs/axum).
* DNS Listener - A simple implementation of a DNS listener.

If you haven't guessed already, I'm basically using the project to test out as many libraries from the tokio boys and girls. Anyway I've been very slowly coding it, doing like a million refactors as I'm slowly getting better at Rust. I was thinking of maybe writing a blog series on it at the very least, with no direct source code. More of a follow along, and if you get it, you should be able to implement something yourself. Just to keep things from getting signatured and give me peace of mind that no one is breaking the law with something I wrote.


Jeremy
