# The Great Suspender Recovery

Ironically enough The Great suspender ended up not being so great,

and got suspended from google extensions store.

For having malicious intent code inside of it after ownership transfer.

Following is the quick, dirty and lazy way to recover your tabs.

## What do

* Acquire safe version

* Build it

* Install it

* Extract session

* Parse URLs

### Acquire safe version

that being the last version uploaded to the github by the original developer

```
git clone --depth 1 --branch v7.1.6 git@github.com:greatsuspender/thegreatsuspender.git
```

### Build it

```
cd thegreatsuspender/

npm install
npm run generate-key
npm run build
```

### Install it

unzip extension bundle

```
cd build/zip
unzip tgs-7.1.6.zip tgs-7.1.6
```

*note: if you don't have developer mode enabled, you are going to have to enable it now*

Inside chrom[e/ium] navigate to the: `chrome://extensions/` and enable `Developer mode`

Click on the `[Load unpacked]` button.

open previously unzipped folder located at `thegreatsuspender/build/zip/tgs-7.1.6` if you folowed everything to the t.

### Extract session

Now you should have working extesion running, so let's actually recover your tab session.

Click on the The Great Suspender logo(or open it up from the `settings->more tools->extensions`)

Click `Settings`, and navigate to the `Session management` tab.

Under `Current session:` you should have expandable session dialog,

you should be able to preview your saved tab session and export it.

example: `5 Feb 2021 7:14am (12 windows, 287 tabs) [export] [save]`

Click `[export]`, that will save session.txt file of the form:
```
...
chrome-extension://<EXTENSION_ID?>#ttl=<TAB_TITLE?>&pos=0&uri=<TAB_URL>
chrome-extension://<EXTENSION_ID?>#ttl=<TAB_TITLE>>&pos=0&uri=<TAB_URL>
...
```

Having tabs with what i assume are `Extension id`, `Tab title`,
and what i am certain of `Tab URI`, separated with new lines.

*note: if you enjoy manual labor or just comfortable enough with your text editor —*

*we are done here, thanks for reading!*

### Parse

We are interested in the latter, that is `tab URI`,

everything that goes after `&pos&uri=` token, up until the end of the line.

Pseudocode:

* Read session.txt file
* Split on the new lines token `'\n'`
* For each element in the collection
  * Find index of the `&pos&uri=` token, by walking.
  * Return new substring `[&pos&uri=.length(), string.length()]`
* Persist new collection of the urls to the fs.

### Implementation

The source code for the script[s], located at the `script/<platform>`

`<platform>` being implementation language

I am providing rust script i've cooked up in a hurry [here](script/rust)

If you feel charitable — contribute to the repository with the recovery script

implementation in programming language of your choice, for others to use.

## Thanks

Thanks for reading!

I am glad that you have your tabs back, cya!

## license

BSD 3-Clause License

Copyright (c) 2021, Kirill Morozov (moki) <morozov.kirill.moki@gmail.com>
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
