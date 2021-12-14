---
title: Google Authentication Update
date: January 28, 2020
tags: wildwood, google, authentication
series: security
---

# Google Authentication Update

Good morning Wildwood and WCA Staff! We got some news last week in regards to a Google Authentication update; I just want to inform you on how this affects your staff @wildwoodcalvary.com email and then any personal Gmail emails you may use. With the craziness of internet security management and people getting hacked all the time, Google is pushing (forcing) and updating everyone to their ‘OAuth2’ security system. This will affect every Gmail associated email in regards to ‘third-party clients’; this means any software that has access your Gmail, that isn’t created by Google. All @wildwoodcalvary.com emails are Gmail accounts and are affected by this. If you only access your Gmail accounts from gmail.com or the Gmail app and use an Android this won’t affect you in any way. Side note, I don’t know if this applies to anyone but want to cover everything; if your developing software with Google APIs I will link the updated documents for upgrading your clients to ‘OAuth2’. Alright let’s jump into the fun; I will split instructions based on platform so you can jump to what you use; this will cover computers and phones.

## Apple MacBook Users

On Apple computers, Apple pre-installs their Email, Calendar and Contact apps; they are popular and good apps. If you have any Gmail account linked into these programs they will be affected by the security change, luckily Apple has already updated their Apps to support the upgrade. The only thing you need to do is remove the Gmail account from the app and add it back, selecting ‘Google Account’ when you add it back. If you connected your Gmail email to these apps before Apple did the upgrade your account will stop syncing on June 15, 2020. At that point you will just need to remove the account and add it back. If your unsure if your using the latest security you can remove and add your account back right now; or just wait until June 15th to re-add it.

## Windows Users

If you’re on Windows the only build-in application that will be affected is the Mail app. If you use the mail app you will need to remove your Gmail account and add it back to move to the new security update. Microsoft has already updated its app to support the new security update; so, you can re-add your account right now. Syncing for the old update will stop on June 15, 2020.

## iOS Users

Just like with the Apple MacBook Applications, on your iPhone/iPad you will need to remove your account from your devices mail settings, in the settings app, and re-add your Gmail account. You are free to do that now, as iOS is currently updated to support the new security update. Please don’t remove your iCloud account, you only need to remove your Gmail from your list of emails under the ‘mail’ settings on the iPhone and then add it back.

## Android Users

You don’t have to do anything; Google makes Android.

## Outlook Users (Mac and Windows)

If you use Outlook on any operating system you will need to upgrade to Outlook 2019 or Office 365 to use the new Google security update. If you are using versions 2016 or lower your Gmail email will stop syncing on June 15, 2019. If you’re on an older version and use Outlook please talk to me and I will upgrade you to Office 2019. We do not use Office 365 here at Wildwood, so that would have to be something you pay for personally if you want it.

## Thunderbird and/or other Third-Party Apps (Mac and Windows)

You will need to remove your current Gmail email account and re-add it back using ‘IMAP with OAuth’. Make sure whatever third-party client you are using is updated to the latest version it supports.

## Other Cases

If you are having any issues with your Gmail accounts syncing please let me know so I can help you sort it out.

## Development

If you are developing software or have already developed software that uses Google APIs you will need to upgrade those as well to the new APIs. <a href="https://developers.google.com/identity/protocols/OAuth2WebServer" target="_blank">Here is the link to the documents</a>.

## Conclusion

Sorry, I know this is kind of annoying and there are a ton of weird use cases but it is ultimately for better security which benefits us all in the long run. It further prevents our work emails from being attacked, keeping us safer. If you have any questions or something was confusing please let me know. This all comes in affect on June 15, 2020 and will be finalized on February 15, 2021.

### Eric Christensen
