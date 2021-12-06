use super::{command::Command, folder::Folder};

#[derive(Default)]
pub struct FolderComposite<Folder1, Folder2> {
    folder_1: Folder1,
    folder_2: Folder2,
}

impl<Folder1: Folder, Folder2: Folder> Folder for FolderComposite<Folder1, Folder2> {
    type Output = (Folder1::Output, Folder2::Output);

    fn apply(self, command: Command) -> Self {
        Self {
            folder_1: self.folder_1.apply(command.clone()),
            folder_2: self.folder_2.apply(command),
        }
    }

    fn unwrap(self) -> Self::Output {
        (self.folder_1.unwrap(), self.folder_2.unwrap())
    }
}
