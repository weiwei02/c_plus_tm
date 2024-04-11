package alstring

import "testing"

func Test_stringMutiply(t *testing.T) {
	type args struct {
		str1 string
		str2 string
	}
	tests := []struct {
		name string
		args args
	}{
		{
			"0",
			args{
				"1234",
				"0",
			},
		},
		{
			"56088",
			args{
				"123",
				"456",
			},
		}, {
			"400",
			args{
				"10",
				"40",
			},
		}, {
			"1001",
			args{
				"1001",
				"1",
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := stringMutiply(tt.args.str1, tt.args.str2); got != tt.name {
				t.Errorf("stringMutiply() = %v, want %v", got, tt.name)
			}
		})
	}
}
