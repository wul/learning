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
(setq liwu "LI WU")
(symbolp 'liwu)
(symbolp "liwu")
(hello 'liwu)
(hello liwu)
(symbol-name 'liwu)
(symbol-function 'liwu)
(symbol-function 'hello)

(defun foo (b1 b2)
  (interactive "bBuffer to rename:\nsRename buffer %s to:")
  (message "we change %s to %s" b1 b2))

(foo 'zs)


(defun inact (one two)
  (interactive "r")
  (message "we got %s and %s" one two))



(interactive-form 'inact)

	      
(defun func (arg start end)
  (interactive "P\nr") 
  (message "Your input is: %d, %s %s" arg start end))

(interactive-form 'func)
