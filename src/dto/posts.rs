api_errors!(
    PostError,
    responses(
        PostNotFound = (
            status = NOT_FOUND,
            description = "Could not find the post.",
            variants = (PostNotFound = "Post not found.")
        ),
        ThreadNotFound = (
            status = NOT_FOUND,
            description = "Could not find the thread.",
            variants = (ThreadNotFound = "Thread not found.")
        ),
    )
);

