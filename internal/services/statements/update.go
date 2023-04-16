package statements

const ConfirmAccount = `
	UPDATE users
	SET is_confirmed = 1
	WHERE email = ?`
