

(defun pg-select (groups group)
	(cond
		((null (car groups)) nil)
		((eql (caar groups) group) (pg-select1 (car groups)))
		(t (pg-select (cdr groups) group))))

(defun pg-select1 (group)
	(char (cadr group) (random (length (cadr group))))
)


(defun pg-generate (groups template)
	(labels
		((generate (groups template password)
			(if (null template) password
				(generate groups (cdr template)
					(concatenate 'string password
						(string (pg-select groups (car template))))))
		))
		(generate groups template nil)))


(defun pg-create nil (cons nil nil))

(defun pg-insert (groups group)
	(rplacd groups (cons (car groups) (cdr groups)))
	(rplaca groups group)
)


(set 'groups (pg-create))

(pg-insert groups '(v "aeiou"))
;(pg-insert groups '(v "aeiouAEIOU"))
(pg-insert groups '(c "bcdfghjklmnpqrstvxyz"))
;(pg-insert groups '(c "bcdfghjklmnpqrstvxyzBCDFGHJKLMNPQRSTVXYZ"))
(pg-insert groups '(d "0123456789"))
(pg-insert groups '(s "`~!@#$%^&*()[{]}'\",<.>/?=+-_\\|;:"))
(pg-insert groups '(_ " " ))

(set 'template '(c v c v _ c v c v _ c v c v _ c v c v))
;(set 'template '(c v c v _ d d s _ c v c v _ d d s _ c v c v _ d d s _ c v c v _ d d s _ c v c v _ d d s _ c v c v _ d d s))
;(set 'template '(c v c v _ d d s _ c v c v _ d d s _ c v c v _ d d s))
;(set 'template '(c v c v _ c v c v _ c v c v _ d d d d))
;(set 'template '(c v c v _ c v c v))


(set '*random-state* (make-random-state t))

(dotimes (i 1000)
	(let ((p (pg-generate groups template)))
		(format t "~a~&" p)
		;(format t "<a href=\"http://google.com/search?hl=en&q=~a\">~a</a><br/>~&" p p)
	))
