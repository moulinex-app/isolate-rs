use crate::IsolatedContainerBuilder;

#[test]
fn test() {
    let container = IsolatedContainerBuilder::new(0)
        .wall_clock_timeout(60)
        .memory_limit(100_000)
        .build();

    container.run(vec!["/bin/sh", "-c", "echo Hello World!"]);
    container.delete();
}