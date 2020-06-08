package file

import "os"

// CheckOrCreateFile ...
func CheckOrCreateFile(path string) error {
	_, err := os.Stat(path)
	if err != nil {
		if os.IsNotExist(err) {
			file, err := os.Create(path)
			if err != nil {
				return err
			}
			defer file.Close()
		} else {
			return err
		}
	}
	return nil
}
