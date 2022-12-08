from dataclasses import dataclass
from typing import Optional

FILE_SIZE_LIMIT = 100000


@dataclass
class File:
    size: int


@dataclass
class Folder:
    files: list[File]
    folders: dict[str, "Folder"]
    parent: Optional["Folder"]

    def get_sizes(self, contents: list) -> int:
        self_files = sum(file.size for file in self.files)
        self_folders = sum(
            folder.get_sizes(contents) for folder in self.folders.values()
        )
        contents.append(self_files + self_folders)
        return self_files + self_folders


def main() -> None:
    cd = Folder([], {}, None)
    all_folders: list[int] = []

    while True:
        tokens = input().split()
        match tokens:
            case ["$", "cd", "/"]:
                while cd.parent is not None:
                    cd = cd.parent
            case ["$", "cd", ".."]:
                cd = cd.parent or cd
            case ["$", "cd", name]:
                if name in cd.folders:
                    cd = cd.folders[name]
                else:
                    cd.folders[name] = Folder([], {}, cd)
                    cd = cd.folders[name]
            case ["$", *_]:
                pass
            case ["dir", name]:
                if name not in cd.folders:
                    cd.folders[name] = Folder([], {}, cd)
            case [size, _]:
                cd.files.append(File(int(size)))
            case ["sum"]:
                while cd.parent is not None:
                    cd = cd.parent

                cd.get_sizes(all_folders)
                break
            case _:
                pass

    print("Criterion:", sum(filter(lambda x: x <= FILE_SIZE_LIMIT, all_folders)))


if __name__ == "__main__":
    main()
