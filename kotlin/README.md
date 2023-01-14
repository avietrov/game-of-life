I though this would be a piece of cake, since, well I've been coding java for 10 years.
Turns out, when you build "enterprise apps" you tend to forget the basics, eg. how to build a JAR?

# Setup
I've installed JDK with sdkman!.
And out of principle refuse to use maven / gradle for a 1-file command line app.

# Running
The commands I'm using:
```
kotlinc life.kt -include-runtime -d life.jar
java -jar life.jar
```
