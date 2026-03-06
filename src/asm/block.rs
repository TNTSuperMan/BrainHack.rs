use crate::asm::{asm::AssemblyOp, ctx::AssembleContext};

struct AddrMap {
    block: usize,
    addr: usize,
    temp: usize,
}
impl AddrMap {
    pub fn new(st: usize, dy: usize) -> AddrMap {
        let block = dy + 2;
        let addr = st + block;
        let temp = addr + 1;
        AddrMap {
            block, addr, temp,
        }
    }
}

pub fn assemble_block(ctx: &mut AssembleContext, block: &[AssemblyOp]) {
    for op in block {
        match op {
            AssemblyOp::Move(source, dests) => {
                ctx.go(*source);
                ctx.push("[-");
                for &(dest_ptr, dest_val) in dests {
                    ctx.go(dest_ptr);
                    ctx.add(dest_val);
                }
                ctx.go(*source);
                ctx.push("]");
            }
            AssemblyOp::Set(ptr, val) => {
                ctx.go(*ptr);
                ctx.push("[-]");
                ctx.add(*val);
            }
            AssemblyOp::Add(ptr, val) => {
                ctx.go(*ptr);
                ctx.add(*val);
            }
            AssemblyOp::Out(ptr) => {
                ctx.go(*ptr);
                ctx.push(".");
            }
            AssemblyOp::In(ptr) => {
                ctx.go(*ptr);
                ctx.push(",");
            }
            AssemblyOp::Loop(condition, block) => {
                ctx.go(*condition);
                ctx.push("[");
                assemble_block(ctx, block);
                ctx.go(*condition);
                ctx.push("]");
            }
            AssemblyOp::Fetch(index) => {
                let AddrMap { block, addr, temp } = AddrMap::new(ctx.st(), ctx.dy());

                // 目的地に移動
                ctx.go(addr);
                ctx.push("[");

                ctx.go(addr + block); // アドレス転送先をクリア
                ctx.push("[-]");

                ctx.go(addr); // アドレスをデクリメント
                ctx.push("-");
                
                ctx.push("[-"); // アドレスをmove
                ctx.go(addr + block);
                ctx.push("+");
                ctx.go(addr);
                ctx.push("]+"); // マーク

                ctx.go(addr + block); // ズラす
                ctx.pointer = addr;

                ctx.push("]");

                // 値をtempに移動
                let val = addr - 1 - index;

                ctx.go(val); // valをaddr(ゼロ状態)にmove
                ctx.push("[-");
                ctx.go(addr);
                ctx.push("+");
                ctx.go(val);
                ctx.push("]");

                ctx.go(addr); // addrをval/tempにmove
                ctx.push("[-");
                ctx.go(val);
                ctx.push("+");
                ctx.go(temp);
                ctx.push("+");
                ctx.go(addr);
                ctx.push("]");

                // ZEROマークに戻るまでtemp移動
                ctx.go(addr);
                ctx.push("+["); // マーク付き、ループ開始

                ctx.go(temp - block); // 転送先tempクリア
                ctx.push("[-]");

                ctx.go(temp); // tempをmove
                ctx.push("[-");
                ctx.go(temp - block);
                ctx.push("+");
                ctx.go(temp);
                ctx.push("]");

                ctx.go(addr - block); // ズラす
                ctx.push("]");
            }
            AssemblyOp::Send(index) => {
                let AddrMap { block, addr, temp } = AddrMap::new(ctx.st(), ctx.dy());

                // 目的地に移動
                ctx.go(addr);
                ctx.push("[");

                ctx.go(addr + block); // アドレス転送先をクリア
                ctx.push("[-]");
                ctx.go(temp + block); // 値転送先をクリア
                ctx.push("[-]");

                ctx.go(temp); // 値をmove
                ctx.push("[-");
                ctx.go(temp + block);
                ctx.push("+");
                ctx.go(temp);
                ctx.push("]");

                ctx.go(addr); // アドレスをデクリメント
                ctx.push("-");
                
                ctx.push("[-"); // アドレスをmove
                ctx.go(addr + block);
                ctx.push("+");
                ctx.go(addr);
                ctx.push("]+"); // マーク

                ctx.go(addr + block); // ズラす
                ctx.pointer = addr;

                ctx.push("]");

                // tempから移動
                let val = addr - 1 - index;

                ctx.go(val); // valをクリア
                ctx.push("[-]");

                ctx.go(temp); // move
                ctx.push("[-");
                ctx.go(val);
                ctx.push("+");
                ctx.go(temp);
                ctx.push("]");

                // ZEROマークに戻る
                ctx.go(addr);
                ctx.push("+["); // マーク付き、ループ開始
                ctx.go(addr - block); // ズラす
                ctx.push("]");
            }
        }
    }
}
