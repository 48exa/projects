require("dotenv").config();
const { Client, IntentsBitField } = require("discord.js");

const client = new Client({
   intents: [
      IntentsBitField.Flags.Guilds,
      IntentsBitField.Flags.GuildMembers,
      IntentsBitField.Flags.GuildMessages,
      IntentsBitField.Flags.MessageContent,
   ],
});

client.on("ready", (c) => {
   console.log(`${c.user.username} is ready.`);
});

client.on("interactionCreate", (interaction) => {
   if (!interaction.isChatInputCommand()) return null;

   switch (interaction.commandName) {
      case "hey":
         interaction.reply("hey");
         break;
      case "id":
         interaction.reply("Your id is: " + "`" + interaction.user.id + "`");
         break;
   }
});

client.login(process.env.TOKEN);
