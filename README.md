# fightbot
Simple bot for queuing 2v2-4v4's with minimal permissions.  

This bot handles differently sized queues without needing multiple channels, and more features are being planned.  
Reasons for making our own instead of using PLAG's or Kaine's APES bot:
 - Open source.
 - Uses app commands (slash commands) in Discord. This means it doesn't actually need to read messages in your server, unlike the other bots that use "?" and other triggers.
 - Minimal permission requirement. We disliked that the aforementioned alternatives required global permission to view every channel in your server and read every message in them.

# Installing the Bot  
You may use the below link to add the bot to your server. The only default permission it requires is Send Messages.  
Do not allow bots more default permissions than this unless you trust the authors to read everything on your server.  
**LINK TO INSTALL:** https://discord.com/oauth2/authorize?client_id=1399703839656902718  

Once added to your server, it will create a role for itself. In order to make it functional, you must add either the Bot (as a user) **or** it's role to the channel or channels you would like it to function in, and grant it the **View Channel** and **Send Messages** permissions.  
You must then add the **Use Application Commands** permission to any user role you would like to be able to use the Bot in that channel.


# How To Use

It uses "/" as the trigger. Currently the following possible commands are available:
 - /reg
 - /start
 - /rm

The first function `/reg` will present a text field that asks for `team_size` with the following options:  
 - Enter `2` for 2v2: `/reg` `team_size: 2`  
 - Enter `3` for 3v3: `/reg` `team_size: 3`  
 - Enter `4` for 4v4: `/reg` `team_size: 4`
   
`/reg` has a further possibility of adding a parameter for `user` if you would like to register someone else for a queue.
 - Example Use: `/reg` `team_size: 4` `user: @frogvoid`

When a queue size reaches its maximum threshold (4 for 2v2, etc), the bot will automatically post a comment listing the randomized team breakdown.  

The second function `/start` will ask for `team_size` and will force a return of the team breakdown post in the event of uneven team sizes, eg if you have only 7 people queued for a 4v4.

The third function `rm` is to remove a user from a queue and will ask for `user`  
 - Example Use: `rm` `user: @frogvoid`
