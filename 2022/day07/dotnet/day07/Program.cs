using day07;
using Directory = day07.Directory;
using File = System.IO.File;

var filename = args[0];

var reader = File.ReadLines(filename);

var filesystem = new Directory {Name = "/"};
var current = filesystem;

foreach (var line in reader)
{
    var words = line.Split(" ");

    switch (words[0])
    {
        case "$":
            if (words[1] == "cd")
                current = words[2] switch
                {
                    ".." => current.Parent!,
                    "/" => filesystem,
                    _ => current.Directories[words[2]]
                };
            break;
        case "dir":
            current.AddDirectory(words[1]);
            break;
        
        default:
            current.AddFile(words[1], int.Parse(words[0]));
            break;
    }
}

var part1 = filesystem.Select(dir => dir.CalculateSize()).Where(dirSize => dirSize <= 100000).Sum();
Console.WriteLine($"Part 1: {part1}");

var totalsize = filesystem.CalculateSize();
var cleanupSize = totalsize - 40000000;
var part2 = filesystem.Select(dir => dir.CalculateSize()).Where(dirSize => dirSize >= cleanupSize).Min();
Console.WriteLine($"Part 2: {part2}");
