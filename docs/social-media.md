# Social Media 

OpenPowerlifting can optionally associate lifters with their Instagram profiles.

The list of `Name` to `Instagram` associations is tracked by the file [lifter-data/social-instagram.csv](https://gitlab.com/openpowerlifting/opl-datablob/master/lifter-data/social-instagram.csv).

An example of the table format is below:

| Name              | Instagram           |
|-------------------|---------------------|
| Andrey Malanichev | andrey\_malanichev  |
| Dan Green         | dangreenpowerlifter |
| Ed Coan           | eddycoan            |

Note that the table entries are in alphabetical order by Name.

## Adding a new Instagram association using the GitLab Edit Tool

Because the associations are made in a single file, it is particularly convenient to add an Instagram using GitLab's online editor.

To edit the file,

   1. Log into your [GitLab](https://gitlab.com) account.
   2. Browse to [lifter-data/social-instagram.csv](https://gitlab.com/openpowerlifting/opl-data/blob/master/lifter-data/social-instagram.csv).
   3. Click the "Edit" button, located near the top-right corner of the displayed file.
   4. Manually type in a new line, in the format `Name,Instagram`, with no spacing before or after the comma. If you are entering in a single Instagram, please keep the file in alphabetical order by Name. If you are bulk-entering many Instagrams, just enter them in all at the bottom for convenience.
   5. Make sure that there is exactly one empty line at the very bottom of the file.
   6. Scroll to the bottom of the page and in the box that says "Commit changes" write a description of your changes such as "Added instagrams for Dan Green and Ed Coan in alphabetical order" or "I added "x" amount of lifters not in alphabetical order at the bottom of the list" etc. and then click on the "Commit Changes" button.
   7. Copy and paste the title of the new merge request into the box that says "description" and at the end of the message include @sstangl to notify Sean Stangl of your merge request. eg: "Added instagram for Ed Coan in alphabetical order @sstangl"
   8. Check all three checkboxes next to the phrases "Remove source branch when merge request is accepted." "Squash commits when merge request is accepted." and "Allow edits from maintainers."
   9. Last, click the button that says "Submit Merge Request"
