## 荣誉准则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：和GPT讨论过一些语法类问题。

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：代码思路部分除官方文档外未参考任何rcore相关资料，语法部分参考了若干资料。

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

## 总结

需要移植（兼容lab1/2）的地方：
* start_time和syscall_times：tcb中添加，new中初始化，run_task切换进程中更新时间，各个系统调用（位于process.rs和fs.rs）中更新次数
* task_info：new定义，更新
* mmap和munmap：
* 系统调用次数更新：update_syscall_times
* page_table.rs和memory_set.rs和frame_allocator.rs中：一些用于检查的函数
注：
* 第2-4都位于processor.rs（用于处理当前进程相关的内容）中
* 注意实现细节的变化
* 注意crate管理
* 注意注释标出新增功能，以及impl需要文档注释

需要新增的功能：
sys_spawn：
1. 分析参数路径是否合法（参考exec）
2. 返回值是pid/-1
3. tcb impl中，实现spawn
4. spawn中：
    * 参考new+fork+exec
    * 核心就是分配并初始化tcb，然后更新父进程内容，最后切换，
    * 涉及的具体操作如下：
        * tcb及其字段的分配参考new（除了tcbinner的parent）
        * 父进程更新父子关系、状态、最后加入taskmanager
        * 修改processor中的current
        * 销毁exclusive变量并进行__switch

stride：（注意stride scheduling论文中的pass和stride的含义和本实验中相反，这里我们采用的是论文中的定义）
变量：
* tcb中新增stride、prio、pass
* const变量新增BIG_STRIDE
* 变量初始化：
    * prio初始16，pass初始0，stride初始BIG_STRIDE/16
* 变量更新：
    * 每次调度后，更新pass+= stride（在run_task中）
    * 每次set_prio后，更新stride= BIG_STRIDE/new_prio


一个语法错误：
```bash
[kernel] Panicked at src/sync/up.rs:28 already borrowed: BorrowMutError
```

```rust
/// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        // self.ready_queue.pop_front()
        // lab3
        if self.ready_queue.is_empty() {
            return None;
        }
        let mut min_index = 0;
        for (i, tcb) in self.ready_queue.iter().enumerate() {
            let mut inner = tcb.inner_exclusive_access();
            let min_tcb = self.ready_queue[min_index].clone();
            let mut min_inner = min_tcb.inner_exclusive_access();
            if inner.pass < min_inner.pass {
                min_index = i;
            }
        }
        let min_tcb = self.ready_queue.swap_remove_back(min_index);
        min_tcb
    }
```