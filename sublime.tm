
http://ilkinulas.github.io/programming/2016/02/05/sublime-text-syntax-highlighting.html

~/.config/sublime-text/Packages/Theme - One Dark/One Dark.tmTheme

to create new syntax highlighting
create a new sublime syntax file that contains the scope of stuff
and then modify your tm theme which it will specify the scopes from your syntax file

        <dict>
            <key>scope</key>
            <string>rust.custom.String</string>
            <key>settings</key>
            <dict>
                <key>fontStyle</key>
                <string>bold</string>
                <key>foreground</key>
                <string>#F7860A</string>
            </dict>
        </dict>
        <dict>
            <key>scope</key>
            <string>rust.custom.let</string>
            <key>settings</key>
            <dict>
                <key>fontStyle</key>
                <string>bold</string>
                <key>foreground</key>
                <string>#204A87FF</string>
            </dict>
        </dict>