package templates

// ConfirmError encapsulates the information displayed to the user when confirming an account fails.
var ConfirmError = Data{
	PageTitle:    "Confirm",
	ContentTitle: "Confirm Error",
	Content: `An error occurred when you requested to confirm your account.
				The problem has been forwarded to our team automatically. We will look into it and come
                back to you. We apologise for this inconvenience.`,
}

// ConfirmSuccess encapsulates the information displayed to the user when confirming an account succeeds.
var ConfirmSuccess = Data{
	PageTitle:    "Confirm",
	ContentTitle: "Confirmation Successful",
	Content:      "Your account has been confirmed. You will now receive an email whenever there are new heavy metal releases to headbang to.",
}

// NoDuplicateUsersError encapsulates the information displayed to the user when stopping their service fails.
var NoDuplicateUsersError = Data{
	PageTitle:    "Start",
	ContentTitle: "Start Service Error",
	Content: `We are unable to process your request to sign up for our service again using the same email address
				that is currently associated with an active account. Our policy prohibits multiple accounts 
				with the same email address.`,
}

// StopError encapsulates the information displayed to the user when stopping their service fails.
var StopError = Data{
	PageTitle:    "Stop",
	ContentTitle: "Stop Service Error",
	Content: `An error occurred when you requested to stop using our service.
				The problem has been forwarded to our team automatically. We will look into it and come
                back to you. We apologise for this inconvenience.`,
}
