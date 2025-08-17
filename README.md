# fightbot
Simple bot for queuing 2v2-4v4's with minimal permissions.  

This bot handles differently sized queues without needing multiple channels and relies on Discord's slash command interface to avoid needing message read permissions.  

See the [Issue Tracker](https://github.com/FranzTurdinand/fightbot/issues) to view planned upcoming features (and feel free to add your own!)

# Installing the Bot  
You may use the below link to add the bot to your server. It does not require any default global permissions.  
Do not allow bots default permissions unless you trust the authors to read everything on your server.  
[**LINK TO INSTALL**](https://discord.com/oauth2/authorize?client_id=1399703839656902718)  

Once added to your server, go into your **Server Settings,** go to **Integrations** and select the Bot ("Pipcheck"). In this window you may deauthorize the "everyone" role and add any specific roles you want to be able to use the Bot, as well as restrict the bot to a specific channel or channels you want it to work in.
You must then add the **Use Application Commands** permission to any user role you would like to be able to use the Bot in that channel's role settings.


# How To Use

It uses "/" as the trigger. Currently the following possible commands are available:
 - `/main_menu`
 - `/reg`
 - `/start`
 - `/rm`
 - `/cancel`

The `/reg` function will present a text field that asks for `team_size` with the following options:  
 - Enter `2` for 2v2: `/reg` `team_size: 2`  
 - Enter `3` for 3v3: `/reg` `team_size: 3`  
 - Enter `4` for 4v4: `/reg` `team_size: 4`
   
`/reg` has a further possibility of adding a parameter for `user` if you would like to register someone else for a queue.
 - Example Use: `/reg` `team_size: 4` `user: @frogvoid`

When a queue size reaches its maximum threshold (4 for 2v2, etc), the bot will automatically post a comment listing the randomized team breakdown.  

The `/start` function will ask for `team_size` and will force a return of the team breakdown post in the event of uneven team sizes, eg if you have only 7 people queued for a 4v4.

The `/rm` function is to remove a user from a queue and will optionally ask for `user`  
 - Simply run `/rm` and hit enter to remove yourself from any queue you are registered for.
 - Add the `user` parameter to remove someone else, eg `/rm` `user: @frogvoid`

The `/cancel` function is used in conjunction with `team_size` to clear a queue for a given group.

#  
Made by @sesim-nov, @cmdr-wdx, @franzturdinand  
For questions or tech support, contact @desmo1199 on discord
