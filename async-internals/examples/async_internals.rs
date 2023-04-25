use anyhow::Result;
use tokio::{fs, io::AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}

/*
 * 类比golang中的groutine
 * 每个future 是一个协程, future.await 就会由future的调度器 executor来执行
 * 类比golang GMP中的p, 执行时调用future接口的poll方法
 * 当future执行过程中阻塞时(即cpu不被使用时,例如在进行IO操作,这时候cpu就在闲置)
 * 此时poll会返回Poll::Pending, executor会将future挂起,即future阻塞
 * 直到reactor收到了某个事件，然后调用Waker.wake()把future唤醒
 * 这个Waker是Context中的Waker(Context是Waker的一个封装)
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

//Waker内部使用了一个vtable来允许各种各样的waker行为
pub struct Context<'a> {
    waker: &'a Waker,
    _marker: PhantomData<fn(&'a ()) -> &'a ()>.
}

pub struct RawWakerVTable {
    clone: unsafe fn(*const ()) -> RawWaker,
    wake: unsafe fn(*const ()),
    wake_by_ref: unsafe fn(*const ()),
    drop: unsafe fn(*const ()),
}
*/

async fn write_hello_file_async(name: &str) -> Result<()> {
    let mut file = fs::File::create(name).await?;
    file.write(b"hello world!").await?;

    Ok(())
}

/*
 * executor 处理 future时，会不断的调用它的poll方法
 * 实际上 write_hello_file_async("/tmp/hello").await?;
 * 相当于
 * match write_hello_file_async.poll(cx) {
 *      Poll::Ready(result) => return result,
 *      Poll::Pending => return Poll::Pending,
 * }
 *
 * 上面函数里有两个await 代码相当于下面的代码
 *
 * let fut = fs::File::Create(name);
 * match fut.pool(cx) {
 *      Poll::Ready(Ok(file)) => {
 *          let fut = file.write_all(b"hello world!");
 *          match fut.poll(cx) {
 *              Poll::Ready(result) => return result,
 *              Poll::Pending => return Poll::Pending,
 *          }
 *      }
 *      Poll::Pending => return poll::Pending,
 * }
 *
 * async函数返回的是一个future,所以实际上是会把上面的代码封装在一个future实现
 * 里队外提供出去,因此我们需要实现一个数据结构把内部状态保存起来
 * 并为这个数据结构实现Future接口,如下
 * enum WriteHelloFile{
 *      // 初始化阶段,用户提供文件名
 *      Init(String)
 *      // 等待文件创建,此时需要保存Future以便多次调用
 *      AwaitingCreate(impl Future<Output = Result<fs::File, std::io::Error>>),
 *      // 等待文件写入,此时需要保存Future以便多次调用
 *      AwaitingWrite(impl Future<Output = Result<(), std::io::Error>>),
 *      // Future 任务处理完成
 *      Done,
 * }
 *
 * impl WriteHelloFile {
 *      pub fn new(name: impl Into<String>) -> Self {
 *          Self::Init(name.into())
 *      }
 * }
 *
 * impl Future for WirteHelloFile {
 *      type Output = Result<(), std::io::Error>;
 *
 *      fn poll(Self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
 *          let thie = self.get_mut();
 *          loop {
 *              match this {
 *                  // 如果状态是Init 就生成create future
 *                  // 并且把状态切换到Awaiting
 *                  // 这里相当于代码里的fs::File::create(name)
 *                  // 此时还没有调用(即还没有.await)
 *                  WriteHelloFile::Init(name) => {
 *                      let fut = fs::File::create(name);
 *                      *self = WriteHelloFile::AwaitingCreate(fut);
 *                  }
 *
 *                  // 如果状态是AwaitingCreate 那么 poll create future
 *                  // 执行这个future，如果执行返回了Poll::Ready(Ok(_))
 *                  // 那么该future执行完毕,创建write Future,
 *                  // 并且把状态切换到Awaiting
 *                  // 这里match fut.poll 相当于create future的执行(即.await)
 *                  WriteHelloFile::AwaitingCreate(fut) => match fut.poll(cx) {
 *                      Poll::Ready(Ok(file)) => {
 *                          // 这里相当于代码里第一个await执行完毕并且成功
 *                          // 之后的下一句的future的创建file.write_all(..)
 *                          // 此时这个future还没有执行,即还没有.await
 *                          let fut = file.write_all(b"hello world!");
 *                          *self = WriteHelloFile::AwaitingWrite(fut);
 *                      }
 *                      Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
 *                      Poll::Pending => return Poll::Pending,
 *                  }
 *
 *                  // 如果状态是AwaitingWrite, 那么poll write future
 *                  // 这里match fut.poll(cx)开始执行,即第二个.await
 *                  // 如果返回Poll::Ready(_) 那么状态切换到Done
 *                  // 整个future执行成功
 *                  WriteHelloFile::AwaitingWrite(fut) => match fut.poll(cx) {
 *                      Poll::Ready(result) => {
 *                          *self = WriteHelloFile::Done;
 *                          return Poll::Ready(result);
 *                      }
 *                      Poll::Pending => return Poll::Pending,
 *                  }
 *
 *                  // 如果状态是Done 整个Future执行完毕
 *                  WriteHelloFile::Done => return Poll::Ready(Ok(())),
 *              }
 *          }
 *      }
 * }
 *
 * fn write_hello_file_async(name: &str) -> WriteHelloFile {
 *      WriteHelloFile::name(name)
 * }
 *
 */

/*
 并发任务运行在 Future 这样的协程上时，
 async/await 是产生和运行并发任务的手段，
 async 定义一个可以并发执行的 Future 任务，
 await 触发这个任务并发执行。
 具体来说：当我们使用 async 关键字时，它会产生一个 impl Future 的结果。
 对于一个 async block 或者 async fn 来说，内部的每个 await 都会被编译器捕捉，
 并成为返回的 Future 的 poll() 方法的内部状态机的一个状态。
 Rust 的 Future 需要异步运行时来运行 Future，
 以 tokio 为例，它的 executor 会从 run queue 中取出 Future 进行 poll()，
 当 poll() 返回 Pending 时，这个 Future 会被挂起，
 直到 reactor 得到了某个事件，唤醒这个 Future，
 将其添加回 run queue 等待下次执行。
 tokio 一般会在每个物理线程（或者 CPU core）下运行一个线程，
 每个线程有自己的 run queue 来处理 Future。
 为了提供最大的吞吐量，tokio 实现了 work stealing scheduler，
 这样，当某个线程下没有可执行的 Future，
 它会从其它线程的 run queue 中“偷”一个执行。
*/
