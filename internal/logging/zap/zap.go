package zap

import (
	"feedback/internal/logging"
	"feedback/internal/utils/file"
	"fmt"
	"net/http"

	zapsource "go.uber.org/zap"
	"go.uber.org/zap/zapcore"
	"gopkg.in/natefinch/lumberjack.v2"
)

// Zap ...
type Zap struct {
	logger *zapsource.Logger
}

var _ logging.Logger = (*Zap)(nil)

// NewLogger ...
func NewLogger(logFilePath string) (*Zap, error) {
	if err := file.CheckOrCreateFile(logFilePath); err != nil {
		return nil, err
	}

	w := zapcore.AddSync(&lumberjack.Logger{
		Filename:   logFilePath,
		MaxSize:    500, // megabytes
		MaxBackups: 3,
		MaxAge:     28, // days
	})
	config := zapsource.NewProductionEncoderConfig()

	core := zapcore.NewCore(
		zapcore.NewJSONEncoder(config),
		w,
		zapsource.InfoLevel,
	)

	zap := zapsource.New(core)
	defer zap.Sync()

	return &Zap{logger: zap}, nil
}

// Error ...
func (z *Zap) Error(msg string) error {
	z.logger.Error(msg)
	return nil
}

// Info ...
func (z *Zap) Info(msg string) error {
	z.logger.Info(msg)
	return nil
}

// InfoHandler ...
func (z *Zap) InfoHandler(w http.ResponseWriter, r *http.Request) {
	z.logger.Info(fmt.Sprintf("%v", r.Method),
		zapsource.String("remote", r.RemoteAddr),
		zapsource.String("uri", r.RequestURI),
	)
}

// PanicHandler ...
func (z *Zap) PanicHandler(w http.ResponseWriter, r *http.Request, err interface{}) {
	z.logger.Panic(fmt.Sprintf("%v", err),
		zapsource.String("remote", r.RemoteAddr),
		zapsource.String("uri", r.RequestURI),
		zapsource.String("method", r.Method),
	)
	http.Error(w, http.StatusText(http.StatusInternalServerError), http.StatusInternalServerError)
}
