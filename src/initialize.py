import libtmux, time
server = libtmux.Server()
session = server.new_session(session_name="c", )
window = session.attached_window
pane = window.split_window(attach=True)
pane.send_keys("ls")
time.sleep(20)
session.kill_session()
