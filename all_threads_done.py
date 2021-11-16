def all_threads_done(threads):
    for thread in threads:
        if thread.done()==False:
            return False
    return True