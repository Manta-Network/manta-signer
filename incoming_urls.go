package main

import (
	"net/url"
	"strings"

	"github.com/pkg/errors"
)

type incomingURL struct {
	Action string
	Params url.Values
}

func parseIncomingURL(urlStr string) (incomingURL, error) {
	var inURL incomingURL
	u, err := url.Parse(urlStr)
	if err != nil {
		return inURL, err
	}
	if u.Scheme != "manta" {
		return inURL, errors.New("not an manta:// url")
	}
	inURL.Action = strings.Trim(u.Path, "/")
	inURL.Params = u.Query()
	switch inURL.Action {
	case "show":
	default:
		return inURL, errors.Errorf("unsupported action %q", inURL.Action)
	}
	return inURL, nil
}
