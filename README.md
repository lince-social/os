# Anicca
Tool for simulating NixOS impermanence and persistance.

# Install
Download the repo, then run:

```bash
cargo run
```
Or download the latest Release's binary and then ./anicca

# Configuration
The path to the configuration file is in your config dir + anicca/anicca

In Linux (Arch with Archinstall btw), the only system I tested it, the path would be ~/.config/anicca/anicca.

Inside of that file each line will be treated in this way:

> One dir up that line is put into a 'list' of dirs that will have everything in it removed, except the dir specified.

Let's say that the name of my user is 'myname', in this case the Home Directory would be '/home/myname', lets check out our mock files:

```linuxhome
/home/myname/.config/anicca
/home/myname/.config/helix
/home/myname/.config/nvim
/home/myname/.cache
/home/myname/Downloads
```

If Anicca's config file's contents are:

```anicca
/home/myname/.config
```

Every dir inside 'home/myname' except for .config, it's subdirectories and files will be deleted. The result will be this:

```linuxhome
/home/myname/.config/anicca
/home/myname/.config/helix
/home/myname/.config/nvim
```
---

Great! Now let's say that Anicca's config is this:

```anicca
/home/myname/.config/anicca
/home/myname/.config/helix
/home/myname/Downloads
```

Then the result will be this:

```linuxhome
/home/myname/.config/anicca
/home/myname/.config/helix
/home/myname/Downloads
```

When we add one level of 'immersion' inside subdirs we make every 'sibling' subdir also elegible for removing. In the first example with just 'home/myname/.config' the '.config/nvim' dir wasn't specified, therefore it wasn't deleted, all in .config where spared. But in the second case when specific subdirs in '.config' like 'helix' where there the program thought that dirs (and files) inside .config (except for the specified ones: 'anicca' and 'helix') should be deleted. This removed nvim because this hypotetic user thins hypotetically that it's not worth using nvim since it's objectivelly worse than helix.

# TODO
- [ ] Flags for preview of what will be deleted
- [ ] Flag for deleting entire dir without having to specify one phantom subdir
