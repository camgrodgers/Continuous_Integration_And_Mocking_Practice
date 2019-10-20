pipeline {
	agent any
    stages {
        stage('Build') {
            steps {
                sh 'rustup default stable'
                sh 'cargo build --release'
            }
        }
        stage('Test') {
            steps {
                sh 'cargo test --release'
            }
        }
	}
}

