import libtmux, time, random
def init():
    server = libtmux.Server()
    the_session_name = chr(random.randint(128,255))
    server.new_session(session_name=the_session_name)
    session = server.find_where({"session_name": the_session_name})
    window = session.attached_window
    pane = window.split_window(attach=False,vertical=False)
    pane.send_keys("tmux split")
    server.attach_session(the_session_name)
    server.kill_server()
init()