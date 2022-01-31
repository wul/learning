(buffer-name)
(buffer-file-name)
(current-buffer)
(other-buffer)
(switch-to-buffer (other-buffer))
(setq n (buffer-size)) 
(point)
(buffer-size (other-buffer))
(defun hello (name)
  "This is my first function"
  (message "Hello %s" name))
(hello 'zs)


(defun hello2 (name)
  "This is my second function"
  (if (equal name '"Li Wu")
      (message "hello %s" name)
    (message "hi %s"  name)))


(hello2 "Li Wu")
(hello2 "abc")
(hello2 'abc)
(hello2 'af)


