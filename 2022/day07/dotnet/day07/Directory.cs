using System.Collections;

namespace day07;

public class Directory : IEnumerable<Directory>
{
    public required string Name { get; init; }

    public Directory? Parent { get; init; }
    public Dictionary<string, Directory> Directories { get; } = new();
    public Dictionary<string, File> Files { get; } = new();

    public Directory AddDirectory(string name)
    {
        if (Directories.ContainsKey(name))
            return Directories[name];

        var new_dir = new Directory
        {
            Name = name,
            Parent = this
        };

        Directories.Add(name, new_dir);
        return new_dir;
    }

    public File AddFile(string name, int size)
    {
        if (Files.ContainsKey(name))
            return Files[name];

        var new_file = new File(name, size);

        Files.Add(name, new_file);
        return new_file;
    }

    public int CalculateSize()
    {
        var totalSize = 0;
        foreach (var (_, subdir) in Directories)
            totalSize += subdir.CalculateSize();

        foreach (var (_, file) in Files)
            totalSize += file.Size;

        return totalSize;
    }


    public IEnumerator<Directory> GetEnumerator()
    {
        yield return this;

        foreach (var (_, subdir) in Directories)
        {
            foreach (var d in subdir)
                yield return d;
        }
    }

    IEnumerator IEnumerable.GetEnumerator()
    {
        return GetEnumerator();
    }

    public record File(String Name, int Size);
}
