package statements

const DeleteUnconfirmedUsers = `
	DELETE
	FROM users
	WHERE is_confirmed = 0`

const DeleteUser = `
	DELETE
	FROM users
	WHERE email = ?`
