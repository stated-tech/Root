// First, let's connect to the API of google.

const { google } = require('googleapis');
const fs = require('fs');
const credentials = require('./credentials.json');
const scopes1 = ['https://www.googleapis.com/auth/documents.readonly',  'https://www.googleapis.com/auth/drive']; // Here is to say we will access to google drive, and google doc
const auth = new google.auth.JWT(
  credentials.client_email, null,
  credentials.private_key, scopes1
); // We get all authorization with credentials json

const doc = google.docs({ version: "v1", auth }); // We are connected to google doc.

  (async function () {
    let res = await new Promise((resolve, reject) => { // We get the file we want with documentId through a promise
        doc.documents.get({
            documentId: "1BojWg38ZUk80SvyKNtpFJLAhmRGI7Ra5FudHC67KCc0",
          }, function (err, res) {
            if (err) {
              reject(err);
            }
              resolve(res);
          });
        });

let subject_list=[]; // We want to get the list of subject
for(let i= 1; i<res.data.body.content.length;i++){
    if(res.data.body.content[i].paragraph.bullet){// The list of subject come from paragraph.bullet (so if true, we will take it)
        subject_list.push(res.data.body.content[i].paragraph.elements[0].textRun.content); //We take the element which are the subject title
    } 
}
for (let i=0;i<subject_list.length;i++){ // We remove any space 
    subject_list[i]=subject_list[i].split('\n')[0];
}

// Now let's get all the content, inside the google drive

const drive = google.drive({ version: "v3", auth });

  (async function () {// we connect to the google drive and will get all comments ! 
    let res = await new Promise((resolve, reject) => {
      drive.comments.list({
        fileId: "1BojWg38ZUk80SvyKNtpFJLAhmRGI7Ra5FudHC67KCc0",
        fields:"*"
      }, function (err, res) {
        if (err) {
          reject(err);
        }
          resolve(res);
      });
    });

//Now, let's organize the results collected

let content =[]; // A table that will take all comments by order
let author=[]; // By order of subject, the name of author
let nb_comments_per_subject=[]; // index represent title 1, 2 etc, and the table represent the number of comments for each title
let count=0;
for (let i=0;i<subject_list.length;i++){// for each subject, we will add the content in content, same with author and will count the number of comment for each subject
    for(let j=0;j<res.data.comments.length;j++){
        if(res.data.comments[j].quotedFileContent.value ==subject_list[i]){
            content.push(res.data.comments[j].content);
            author.push(res.data.comments[j].author.displayName);
            count=count+1;
        }
    }
    nb_comments_per_subject[i]=count;
    count=0;
}

console.log(nb_comments_per_subject);
let temp_comment = [];
let temp_author = [];
let count_comment =0;


for (let i=0;i<nb_comments_per_subject.length;i++){ // Let put the comment in the right order
    temp_comment = [];
    temp_author = [];

    for (let j=0;j<nb_comments_per_subject[i];j++) {
        temp_comment.push(content[count_comment+j]);
        temp_author.push(author[count_comment+j]);
    }
    for (let j=0;j<nb_comments_per_subject[i];j++) {
        content[count_comment+j]=temp_comment[nb_comments_per_subject[i]-1-j];
        author[count_comment+j]=temp_author[nb_comments_per_subject[i]-1-j];
    }
    count_comment = count_comment +nb_comments_per_subject[i];
}


//Now we get all content, let's use our bot to create our poll.

require('dotenv').config()
const Discord = require('discord.js');
const { EmbedBuilder } = require('discord.js');

const { Client,  GatewayIntentBits, } = require('discord.js')
const client = new Client({
    intents: [
        GatewayIntentBits.Guilds,
        GatewayIntentBits.GuildMessages,
        GatewayIntentBits.MessageContent
    ]
})

client.on("ready", () => { // With last lines, our bot is readdy (to do that, you have to create it in discord developer), let's create the poll 
    console.log("Bot is ready")
})

let emo = ['1️⃣','2️⃣','3️⃣','4️⃣','5️⃣','6️⃣','7️⃣','8️⃣']; // This will be the emoji to vote for each comment, we assume for each subject, no more than 8
let emo2 = ['👍','👎']; // When there is only one comment, it is not a preferundum but a referundum that's the reason of those two emoji

client.on("messageCreate", async message=>{ // Every time a message is create, we will do what follow

    if(message.author.bot || message.channel.type === "dm") return; // if it come from us or a dm, our bot don't do anything

    const messageArray = message.content.split(' '); // we take the message
	const cmd = messageArray[0];
	const args = messageArray.slice(1);

    let it=0; // we will iterate on all comment with that

    for(let i=0;i<subject_list.length;i++){
        if (cmd === '/vote'){ // When it is written /vote, it will do what follows, orelse any message will have impact
            let pollDescription = subject_list[i]; // for each subject, the poll description will be of course the subject title itself
        
            let embedPoll = new EmbedBuilder() // We create the first part of the poll, with a description and format we want
            .setTitle(pollDescription.toUpperCase())
            //.setDescription('toto \n tata')
            .setColor(0x0099FF)

            let embedPoll2 = new EmbedBuilder() // We create this to end a poll with the vote
            .setDescription('VOTE HERE FOR' +" "+subject_list[i].toUpperCase())

            let msgEmbed = await message.channel.send({ embeds: [embedPoll] }); // We send the first part 

            for (let j=0; j<nb_comments_per_subject[i];j++){
                
                if(nb_comments_per_subject[i]>1){ // for each subject, we put the author and content and emoji if more that one comment
                    message.channel.send(emo[j]+" "+ "by" +" "+author[it]+" "+ "\n"+content[it])
                    it=it+1; // we use that to always display the good comment
                }else{ // if only one comment, we just display the comment without author and emoji
                    message.channel.send(content[it])
                    it=it+1; // we use that to always display the good comment
                }

            }

            if(nb_comments_per_subject[i]>=1){ 
                let msgEmbed2 = await message.channel.send({ embeds: [embedPoll2] }); // with this, we will begin the poll


                for (let j=0; j<nb_comments_per_subject[i];j++){
                
                    if (nb_comments_per_subject[i]>1) { // if there are more than 1 comment, we loop on the number of comment to add as many emoji as needed

                        await msgEmbed2.react(emo[j]) 

                    } else { // if only one comment, we make a referundum with those two emoji

                        await msgEmbed2.react(emo2[0]) 
                        await msgEmbed2.react(emo2[1]) 
                    }
                }
            }
    }
 }
});
client.login(process.env.BOT_TOKEN)
})()

})()

