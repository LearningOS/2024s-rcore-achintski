## 荣誉准则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：和GPT讨论过一些语法类问题。

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：代码思路部分除官方文档外未参考任何rcore相关资料，语法部分参考了若干资料。

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

## 总结
### 文件系统
目的：持久化存储
本质：对外存的抽象，位于块设备层上（文件系统的操作归根结底落在块设备上，块设备需要块设备驱动）
核心：内存中/外存中数据结构以及中间的抽象层
技能点：快速上手工程类任务的代码
过程：了解新框架（尤其注意本章代码导读）+兼容已有功能（在本lab4中，只需要修改sys_spawn中模仿sys_exec处理参数的部分，其余和lab3相同）+添加新功能（实现三个系统调用 sys_linkat、sys_unlinkat、sys_stat）

### 框架分析：思考读/写一个文件时发生了什么？
easy-fs/src：block_dev-->block_cache-->layout-->efs-->vfs
1. 磁盘块设备接口层：/block_dev.rs

    * 归根结底是在块设备上以块为单位读写

    * 读写磁盘块设备的trait接口-- BlockDevice trait（仅需read_block 和 write_block）

2. 块缓存层：/block_cache.rs
    * 缓冲区是块设备的上一层，以块为单位管理对应的块缓存
    * BlockCache：创建时会触发read_block
    * BlockManager：以类似FIFO方式管理BlockCache，被换出时可能触发write_block
    * get_block_cache 接口：通过block_id和block_dev参数，在BlockManager中查询对应的BlockCache，如果存在则直接使用，否则加载（核心是new中的block_device.read_block函数，将编号为 block_id 的块从磁盘读入内存中的缓冲区 buf）进BlockManger
3. 磁盘数据结构层：/layout.rs /bitmap.rs
    * 典型unix布局：超级块+inode位图+data位图+inode分区+data分区
    * 表示磁盘文件系统的数据结构：SuperBlock、Bitmap、BlockInode、DirEntry、DataBlock
    * 注意：
        * 一个BlockCache块缓存对应一个块512B，而一个块中有4个BlockInode
        * 对BlockInode添加新的元数据字段需要修改一级索引的长度，以保证总大小为128B
        * DiskInode 方法：
            * get_block_id：数据块索引功能
            * read_at：将dkinode对应的文件从offset字节开始读到buf中（需要先通过get_block_id及索引定位到块号，然后用get_block_cache读入到内存中）
4. 磁盘块管理器层：/efs.rs
    * 管理磁盘数据结构的控制逻辑
    * EasyFileSystem
    * 注意从这一层开始，所有的数据结构就都放在内存上了
    * 重要方法：
        * get_disk_inode_pos
        * get_data_block_id
5. 索引节点层：/vfs.rs
    * 对单个文件的管理和读写的控制逻辑
    * Inode（why/how对应DiskInode）：通过用efs的get_disk_inode_pos方法和BlockInode的inode_id可以算出该BlockInode所在block的block_id以及磁盘内偏移block_offset，而用get_block_cache接口和block_id以及block_device可以获得对应Block的BlockCache，使用BlockCache的read/modify方法就可以读/写Inode对应BlockInode对应的块缓存中的区域。因此，总的来说定位一个BlockInode需要block_id、block_offset、block_device、fs四个要素，这也正是vfs Inode的组成
    * 重要方法：
        * read/modify_disk_inode：读/写Inode对应的DiskInode对应的BlockCache区域
    * easy-fs-fuse
        * 在（linux上的文件模拟出来的）一个虚拟块设备上创建并初始化文件系统
        
文件系统初始化：（从上到下递进）
1. 打开块设备-- BLOCK_DEVICE
2. 打开块设备上装载的文件系统（磁盘块管理器）-- EasyFileSystem（easy-fs/src/efs.rs）（使用EasyFileSystem::open方法）
3. 从文件系统获取根目录inode--ROOT_INODE

### 新增功能分析--linkat/unlinkat & stat
模仿open操作，参数判断和转换放在os/src/syscall/fs.rs的对应sys_中，接口放到os/src/fs/inode.rs中，具体的操作放到easy-fs/src/vfs.rs中。

具体来说，增加硬链接时，我们需要在目录文件对应的inode（有好多类inode）中添加目录项，并将nlink标记+1，删除硬链接时，我们需要删除目录项，并将nlink标记-1，如果nlink为0，则删除文件；查询状态时，我们需要获得inode_id（注意是磁盘上的）、文件类型以及硬链接数

于是综合上述内容，我们选择把inode_id和nlink放在DiskInode中

同时注意到因为采用了扁平化操作，所以目录文件只有ROOT_INODE对应的根目录文件

### 语法问题
如何将fd_table[fd]从一个trait转换为OSInode：新增AnyConvertor trait，将任意类型转为 &dyn Any 再通过 downcast_ref::<OSInode>() 强制转换为 OSInode 数据类型
